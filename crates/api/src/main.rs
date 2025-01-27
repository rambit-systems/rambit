//! HTTP server for the Rambit API.
//!
//! The API server itself runs on Axum, and serves to run tasks from the `tasks`
//! crate in response to HTTP requests. It also serves as the platform threshold
//! for authentication.
//!
//! # CLI
//! It has two CLI subcommands: `start` and `health`.
//! - `start` starts the API server in regular operation.
//! - `health` runs the health check, dumps it as JSON to `stdout`, and exits.
//!   This is generally for testing configuration in tests.
//!
//! See `api --help` for more information and other options.
//!
//! # Environment Variables
//! It has no extra required environment variables, outside of those required by
//! its services. If you're missing one, it will tell you. Your exact service
//! configuration depends on a number of other crates, in addition to which
//! things you're mocking.

mod cmd;
mod temp_storage_payload;

use std::{sync::Arc, time::Duration};

use axum::{
  body::Body,
  extract::{FromRef, Path, State},
  http::HeaderMap,
  response::{IntoResponse, Response},
  routing::{get, post},
  Json, Router,
};
use clap::Parser;
use miette::{IntoDiagnostic, Result};
use mollusk::ExternalApiError;
use prime_domain::{
  hex::health::{self, HealthAware},
  models,
  repos::{db::Database, TempStorageRepository},
  DynPrimeDomainService,
};
use tasks::Task;
use tracing_subscriber::prelude::*;

use self::{
  cmd::{Commands, RuntimeConfig},
  temp_storage_payload::TempStoragePayload,
};

async fn fetch_handler(
  State(app_state): State<AppState>,
  Path((cache_name, path)): Path<(String, String)>,
  headers: HeaderMap,
) -> Result<Response, ExternalApiError> {
  let token_id_secret_pair = headers
    .get("authorization")
    .and_then(|value| value.to_str().ok())
    .map(|value| value.to_string());
  let token_id = token_id_secret_pair
    .clone()
    .and_then(|pair| pair.split(':').next().map(|s| s.to_string()))
    .and_then(|s| models::TokenRecordId::try_from(s).ok());
  let token_secret = token_id_secret_pair
    .and_then(|pair| pair.split(':').nth(1).map(|s| s.to_string()))
    .map(|s| models::TokenSecret::new(models::StrictSlug::new(s)));

  let data = app_state
    .prime_domain_service
    .fetch_path(
      models::StrictSlug::new(cache_name),
      token_id,
      token_secret,
      models::LaxSlug::new(path),
    )
    .await?;

  let data = data.adapt_to_no_comp();

  Ok(Response::new(Body::from_stream(data)))
}

#[tracing::instrument(skip(app_state, payload))]
async fn naive_upload(
  State(app_state): State<AppState>,
  Path((cache_name, original_path)): Path<(String, String)>,
  payload: TempStoragePayload,
) -> Result<(), mollusk::ExternalApiError> {
  let path = models::LaxSlug::new(original_path.clone());
  if path.to_string() != original_path {
    return Err(
      mollusk::InvalidPathError {
        path: original_path,
      }
      .into(),
    );
  }

  let payload_path = payload.upload().await.unwrap();
  tasks::NaiveUploadTask {
    cache_name: models::StrictSlug::new(cache_name),
    path,
    temp_storage_path: payload_path,
  }
  .run(app_state.prime_domain_service.clone())
  .await
  .unwrap();
  Ok(())
}

async fn dummy_root_handler() -> impl IntoResponse {
  "You've reached the root endpoint of the Rambit API binary.\nYou probably \
   meant to go somewhere else."
}

#[tracing::instrument(skip(app_state))]
async fn health_handler(
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let report = app_state.health_report().await;
  let overall_status = report.overall_status();
  Json(serde_json::json!({
    "report": report,
    "overall_status": overall_status,
  }))
}

#[derive(Clone, FromRef)]
struct AppState {
  prime_domain_service: DynPrimeDomainService,
}

impl AppState {
  async fn build(config: &RuntimeConfig) -> Result<Self> {
    let retryable_kv_store =
      prime_domain::repos::db::kv::KeyValueStore::new_retryable_tikv_from_env(
        5,
        Duration::from_secs(2),
      )
      .await;
    let cache_repo = prime_domain::repos::CacheRepositoryCanonical::new(
      Database::new_from_kv(retryable_kv_store.clone()),
    );
    let store_repo = prime_domain::repos::StoreRepositoryCanonical::new(
      Database::new_from_kv(retryable_kv_store.clone()),
    );
    let token_repo = prime_domain::repos::TokenRepositoryCanonical::new(
      Database::new_from_kv(retryable_kv_store.clone()),
    );
    let entry_repo = prime_domain::repos::EntryRepositoryCanonical::new(
      Database::new_from_kv(retryable_kv_store.clone()),
    );
    let temp_storage_repo: Box<dyn TempStorageRepository> = if config
      .mock_temp_storage
    {
      Box::new(prime_domain::repos::TempStorageRepositoryMock::new(
        std::path::PathBuf::from("/tmp/rambit-temp-storage"),
      ))
    } else {
      let temp_storage_creds = prime_domain::TempStorageCreds::new_from_env()?;
      Box::new(
        prime_domain::repos::TempStorageRepositoryCanonical::new(
          temp_storage_creds,
        )
        .await?,
      )
    };
    let user_storage_repo =
      prime_domain::repos::UserStorageRepositoryCanonical::new();

    let prime_domain_service = prime_domain::PrimeDomainServiceCanonical::new(
      cache_repo,
      entry_repo,
      store_repo,
      token_repo,
      temp_storage_repo,
      user_storage_repo,
    );

    Ok(AppState {
      prime_domain_service: Arc::new(Box::new(prime_domain_service)),
    })
  }
}

#[prime_domain::hex::health::async_trait]
impl health::HealthReporter for AppState {
  fn name(&self) -> &'static str { stringify!(AppState) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![self
      .prime_domain_service
      .health_report()])
    .await
    .into()
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let config = RuntimeConfig::parse();

  let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or(tracing_subscriber::EnvFilter::new("info"));
  let fmt_layer = tracing_subscriber::fmt::layer()
    .with_target(false)
    .with_writer(std::io::stderr);
  let registry = tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer);

  let use_chrome_tracing = match &config.command {
    Commands::Start { chrome_tracing, .. } => *chrome_tracing,
    Commands::Health => false,
  };
  let _guard = match use_chrome_tracing {
    true => {
      let (chrome_layer, guard) =
        tracing_chrome::ChromeLayerBuilder::new().build();
      registry.with(chrome_layer).init();
      Some(guard)
    }
    false => {
      registry.init();
      None
    }
  };

  art::ascii_art!("../../media/ascii_logo.png");

  tracing::info!("starting up");
  tracing::info!("config: {:?}", config);

  tracing::info!("initializing services");

  let state = AppState::build(&config).await?;

  tracing::info!("finished initializing services");
  let health_report = state.health_report().await;
  tracing::info!(
    "service health: {}",
    serde_json::to_string(&health_report).unwrap()
  );
  tracing::info!("overall health: {:#?}", health_report.overall_status());

  let (bind_address, bind_port) = match &config.command {
    Commands::Health => {
      let health_report = state.health_report().await;
      println!("{}", serde_json::to_string(&health_report).unwrap());
      match health_report.overall_status() {
        health::HealthStatus::Ok => std::process::exit(0),
        _ => std::process::exit(1),
      };
    }
    Commands::Start {
      bind_address,
      bind_port,
      ..
    } => (bind_address.clone(), *bind_port),
  };

  tracing::info!("starting server");

  let app = Router::new()
    .route("/health", get(health_handler))
    .route("/naive-upload/:name/*path", post(naive_upload))
    .route("/fetch/:name/*path", get(fetch_handler))
    .route("/", get(dummy_root_handler))
    .with_state(state);

  let bind_address = format!("{bind_address}:{bind_port}");
  let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();

  tracing::info!("listening on `{bind_address}`");
  tokio::spawn(async move { axum::serve(listener, app).await });

  tokio::signal::ctrl_c().await.into_diagnostic()?;

  Ok(())
}
