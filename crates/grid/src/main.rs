//! The server-side entrypoint for Rambit.

#![feature(unwrap_infallible)]

mod handler;

use axum::{self};
use axum_login::AuthManagerLayerBuilder;
use grid_state::AppState;
use miette::{Context, IntoDiagnostic};
use tower_sessions::{
  CachingSessionStore, MemoryStore, cookie::time::Duration,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use self::handler::static_asset_handler;

fn setup_tracing() -> miette::Result<()> {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
  let formatter = fmt::layer();

  tracing_subscriber::registry()
    .with(formatter)
    .with(env_filter.clone())
    .try_init()
    .into_diagnostic()?;

  tracing::info!(env_filter = env_filter.to_string(), "tracing setup");

  Ok(())
}

#[tokio::main]
async fn main() -> miette::Result<()> {
  setup_tracing()?;

  let app_state = AppState::build()
    .await
    .context("failed to build app state")?;

  let router = grid_endpoints::router();

  // add fallback and state
  let router = router
    .fallback(static_asset_handler)
    .with_state(app_state.clone());

  let session_layer = tower_sessions::SessionManagerLayer::new(
    CachingSessionStore::new(MemoryStore::default(), app_state.session_store),
  )
  .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::weeks(1)));
  let auth_layer =
    AuthManagerLayerBuilder::new(app_state.auth_domain, session_layer).build();

  let service = router.layer(auth_layer);

  let addr = "[::]:3000";
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .into_diagnostic()
    .context(format!("failed to bind listener to `{addr}`"))?;
  tracing::info!("bound to http://{}", &addr);

  axum::serve(listener, service)
    .await
    .expect("failed to serve axum server");

  Ok(())
}
