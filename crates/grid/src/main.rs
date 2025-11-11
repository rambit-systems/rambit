#![feature(iterator_try_collect)]

//! The server-side entrypoint for Rambit.

mod app_state;
mod args;
mod endpoints;
mod handlers;
mod middleware;
mod tracing_subscribers;
mod util_traits;

use axum::{Router, handler::Handler, routing::post};
use axum_login::AuthManagerLayerBuilder;
use clap::Parser;
use leptos_axum::LeptosRoutes;
use miette::{Context, IntoDiagnostic, Result};
use tower_http::{
  compression::{CompressionLayer, DefaultPredicate, Predicate},
  trace::{DefaultOnResponse, TraceLayer},
};
use tower_sessions::{
  CachingSessionStore, MemoryStore, cookie::time::Duration,
};
use tracing::{Level, info_span};

use self::{
  app_state::AppState,
  args::CliArgs,
  handlers::{
    leptos_fallback_handler, leptos_routes_handler, server_fn_handler,
  },
  middleware::{
    cache_on_success::CacheOnSuccessLayer,
    compression_predicate::NotForFailureStatus,
  },
};

#[tokio::main]
async fn main() -> Result<()> {
  // set up tracing
  let _guard = self::tracing_subscribers::setup_tracing()
    .context("failed to set up tracing subscribers")?;

  // parse command-line arguments
  let args = CliArgs::parse();

  tracing::info!("starting app server");

  // build app state
  let app_state = AppState::build()
    .await
    .context("failed to build app state")?;

  // prepare leptos
  let routes = leptos_axum::generate_route_list(site_app::App);

  // build router
  let router = Router::new()
    .nest("/api/v1", self::endpoints::router())
    .leptos_routes_with_handler(routes, leptos_routes_handler)
    .route("/api/sfn/{*fn_name}", post(server_fn_handler))
    .fallback(
      leptos_fallback_handler
        .layer(CacheOnSuccessLayer::new())
        .layer(CompressionLayer::new().compress_when(
          DefaultPredicate::new().and(NotForFailureStatus::new()),
        )),
    )
    .with_state(app_state.clone());

  // build tower service
  let trace_layer = TraceLayer::new_for_http()
    .make_span_with(|request: &http::Request<_>| {
      info_span!(
          "http_request",
          method = %request.method(),
          uri = %request.uri(),
      )
    })
    .on_response(DefaultOnResponse::new().level(Level::INFO));

  let session_layer = tower_sessions::SessionManagerLayer::new(
    CachingSessionStore::new(MemoryStore::default(), app_state.session_store),
  )
  .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::weeks(1)))
  .with_secure(!args.no_secure_cookies);
  let auth_layer =
    AuthManagerLayerBuilder::new(app_state.auth_domain, session_layer).build();

  let service = router.layer(trace_layer).layer(auth_layer);

  let addr = format!("{host}:{port}", host = args.host, port = args.port);
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .into_diagnostic()
    .with_context(|| format!("failed to bind listener to `{addr}`"))?;
  tracing::info!("listening on http://{}", &addr);

  tokio::select! {
    result = axum::serve(listener, service) => {
      result
        .into_diagnostic()
        .with_context(|| format!("failed to bind listener to `{addr}`"))?;
    }
    _ = tokio::signal::ctrl_c() => {
      tracing::warn!("received Ctrl+C, shutting down gracefully...");
    }
  }
  tracing::info!("server shut down");

  Ok(())
}
