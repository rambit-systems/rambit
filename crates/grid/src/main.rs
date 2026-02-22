//! The server-side entrypoint for Rambit.

#![feature(unwrap_infallible)]

mod handler;
mod rewrite_root_to_app_root;
mod ulid_request_id;

use axum::{self, Router, middleware};
use axum_login::AuthManagerLayerBuilder;
use grid_state::AppState;
use miette::{Context, IntoDiagnostic};
use tower::ServiceBuilder;
use tower_http::{
  normalize_path::NormalizePathLayer,
  request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
  trace::TraceLayer,
};
use tower_sessions::{
  CachingSessionStore, MemoryStore, cookie::time::Duration,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use self::{
  handler::static_asset_handler,
  rewrite_root_to_app_root::rewrite_root_to_app_root,
  ulid_request_id::MakeRequestUlid,
};

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

  // compose, add fallback, and add state
  let inner_router = app::router()
    .merge(grid_endpoints::router())
    .fallback(static_asset_handler)
    .with_state(app_state.clone());
  // normalize routing (has to happen outside router)
  let router = Router::new().fallback_service(
    ServiceBuilder::new()
      .layer(NormalizePathLayer::trim_trailing_slash())
      .layer(middleware::from_fn(rewrite_root_to_app_root))
      .service(inner_router),
  );

  let session_layer = tower_sessions::SessionManagerLayer::new(
    CachingSessionStore::new(MemoryStore::default(), app_state.session_store),
  )
  .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::weeks(1)));
  let auth_layer =
    AuthManagerLayerBuilder::new(app_state.auth_domain, session_layer).build();

  let layer_stack = ServiceBuilder::new()
    .layer(SetRequestIdLayer::x_request_id(MakeRequestUlid))
    .layer(TraceLayer::new_for_http())
    .layer(PropagateRequestIdLayer::x_request_id())
    .layer(auth_layer);
  let service = router.layer(layer_stack);

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
