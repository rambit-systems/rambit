//! The server-side entrypoint for Rambit.

mod app_state;
mod args;

use axum::{
  Router,
  response::IntoResponse,
  routing::{get, post},
};
use clap::Parser;
use miette::{Context, IntoDiagnostic, Result};
use tower::ServiceBuilder;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

use self::{app_state::AppState, args::CliArgs};

#[axum::debug_handler]
async fn upload() -> impl IntoResponse {}

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
    .route("/upload", post(upload))
    .with_state(app_state);

  let service = ServiceBuilder::new().service(router);

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
