//! The server-side entrypoint for Rambit.

mod app_state;
mod args;

use std::{collections::HashMap, io, str::FromStr};

use axum::{
  Json, Router,
  body::Body,
  extract::{Path, State},
  http::{HeaderMap, StatusCode},
  response::IntoResponse,
  routing::{get, post},
};
use clap::Parser;
use miette::{Context, IntoDiagnostic, Result};
use prime_domain::{
  belt::{self, Belt, StreamExt},
  models::{
    User,
    dvf::{self, EntityName, LaxSlug, RecordId, StrictSlug},
  },
  upload::UploadRequest,
};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, prelude::*};

use self::{app_state::AppState, args::CliArgs};

#[axum::debug_handler]
async fn upload(
  Path(params): Path<HashMap<String, String>>,
  headers: HeaderMap,
  State(app_state): State<AppState>,
  body: Body,
) -> impl IntoResponse {
  let cache_name = params
    .get("cache_name")
    .expect("upload route param names are malformed")
    .clone();
  if dvf::strict::strict_slugify(&cache_name) != cache_name {
    return (
      StatusCode::BAD_REQUEST,
      format!("Cache name is malformed: `{cache_name}`"),
    )
      .into_response();
  }
  let cache_name = EntityName::new(StrictSlug::new(cache_name));

  let desired_path = params
    .get("path")
    .expect("upload route param names are malformed")
    .clone();
  if dvf::lax::lax_slugify(&desired_path) != desired_path {
    return (
      StatusCode::BAD_REQUEST,
      format!("Path is malformed: `{desired_path}`"),
    )
      .into_response();
  }
  let desired_path = LaxSlug::new(desired_path);

  let target_store = params.get("target_store").cloned();
  if let Some(target_store) = &target_store {
    if dvf::strict::strict_slugify(target_store) != *target_store {
      return (
        StatusCode::BAD_REQUEST,
        format!("Target store is malformed: `{target_store}`"),
      )
        .into_response();
    }
  }
  let target_store =
    target_store.map(|ts| EntityName::new(StrictSlug::new(ts)));

  let user_id = match headers.get("user_id") {
    Some(hv) => match hv.to_str() {
      Ok(s) => match RecordId::<User>::from_str(s) {
        Ok(user_id) => user_id,
        Err(_) => {
          return (StatusCode::BAD_REQUEST, "`user_id` malformed: `{s}`")
            .into_response();
        }
      },
      Err(_) => {
        return (StatusCode::BAD_REQUEST, "`user_id` header is not ASCII")
          .into_response();
      }
    },
    None => {
      return (StatusCode::BAD_REQUEST, "`user_id` header missing")
        .into_response();
    }
  };

  let data = Belt::from_stream(
    body
      .into_data_stream()
      .map(|res| res.map_err(|e| io::Error::other(e.to_string()))),
    Some(belt::DEFAULT_CHUNK_SIZE),
  );

  let upload_req = UploadRequest {
    data,
    auth: user_id,
    cache_name,
    desired_path,
    target_store,
  };

  let upload_resp = app_state.prime_domain.upload(upload_req).await;

  match upload_resp {
    Ok(resp) => Json(resp).into_response(),
    Err(err) => format!("{err:?}").into_response(),
  }
}

#[axum::debug_handler]
async fn root() -> impl IntoResponse {
  "You've reached the root endpoint of the Rambit API.\nYou probably meant to \
   go somewhere else."
}

#[tokio::main]
async fn main() -> Result<()> {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(env_filter)
    .init();

  let args = CliArgs::parse();

  tracing::info!("starting app server");

  let app_state = AppState::build()
    .await
    .context("failed to build app state")?;

  if args.migrate {
    app_state
      .prime_domain
      .migrate_test_data(false)
      .await
      .context("failed to migrate test data")?;
  }

  let router: Router<()> = axum::Router::new()
    .route("/", get(root))
    .route("/upload/{cache_name}/{path}", post(upload))
    .route("/upload/{cache_name}/{path}/{target_store}", post(upload))
    .with_state(app_state);

  let service = router.layer(
    TraceLayer::new_for_http()
      .on_response(DefaultOnResponse::new().level(Level::INFO)),
  );

  let addr = format!("{host}:{port}", host = args.host, port = args.port);
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .into_diagnostic()
    .with_context(|| format!("failed to bind listener to `{addr}`"))?;
  tracing::info!("listening on http://{}", &addr);
  axum::serve(listener, service)
    .await
    .into_diagnostic()
    .context("failed to serve app")?;

  Ok(())
}
