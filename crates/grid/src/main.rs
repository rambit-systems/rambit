#![feature(iterator_try_collect)]

//! The server-side entrypoint for Rambit.

mod app_state;
mod args;
mod endpoints;

use axum::Router;
use axum_login::AuthManagerLayerBuilder;
use clap::Parser;
use leptos_axum::LeptosRoutes;
use miette::{Context, IntoDiagnostic, Result};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{Level, info_span, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, prelude::*};

use self::{app_state::AppState, args::CliArgs};

#[tokio::main]
async fn main() -> Result<()> {
  // set up tracing
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(env_filter)
    .init();

  // parse command-line arguments
  let args = CliArgs::parse();

  tracing::info!("starting app server");

  // build app state
  let app_state = AppState::build()
    .await
    .context("failed to build app state")?;

  // migrate if necessary
  if args.migrate {
    match app_state.prime_domain.migrate_test_data(false).await {
      Ok(_) => {
        tracing::info!("migrated test data as requested");
      }
      Err(e) => tracing::warn!("failed to migrate test data: {e}"),
    }
  }

  // prepare leptos
  let leptos_options = app_state.leptos_options.clone();
  let routes = leptos_axum::generate_route_list(site_app::App);

  // build router
  let router = Router::new()
    .nest("/api/v1", self::endpoints::router())
    .leptos_routes(&app_state, routes, {
      let leptos_options = leptos_options.clone();
      move || site_app::shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler::<AppState, _>(
      site_app::shell,
    ))
    .with_state(app_state.clone());

  // build tower service
  let trace_layer = TraceLayer::new_for_http()
    .make_span_with(|request: &axum::http::Request<_>| {
      info_span!(
          "http_request",
          method = %request.method(),
          uri = %request.uri(),
      )
    })
    .on_response(DefaultOnResponse::new().level(Level::INFO));

  let session_layer =
    tower_sessions::SessionManagerLayer::new(app_state.session_store)
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
  axum::serve(listener, service)
    .await
    .into_diagnostic()
    .context("failed to serve app")?;

  Ok(())
}
