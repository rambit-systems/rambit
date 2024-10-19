//! The leptos server crate for the Cartographer app.

use std::sync::Arc;

use axum::{extract::FromRef, Router};
use cart_app::*;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use prime_domain::{
  DynCacheService, DynEntryService, DynStoreService, DynTokenService,
};

#[derive(Clone, FromRef)]
struct AppState {
  cache_service: DynCacheService,
  entry_service: DynEntryService,
  store_service: DynStoreService,
  token_service: DynTokenService,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
  let filter = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or(tracing_subscriber::EnvFilter::new("info"));
  tracing_subscriber::fmt().with_env_filter(filter).init();

  let conf = get_configuration(None).unwrap();
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  let routes = generate_route_list(App);

  let tikv_adapter =
    Arc::new(prime_domain::repos::db::TikvAdapter::new_from_env().await?);
  let cache_repo =
    prime_domain::repos::CacheRepositoryCanonical::new(tikv_adapter.clone());
  let entry_repo =
    prime_domain::repos::EntryRepositoryCanonical::new(tikv_adapter.clone());
  let store_repo =
    prime_domain::repos::StoreRepositoryCanonical::new(tikv_adapter.clone());
  let token_repo =
    prime_domain::repos::TokenRepositoryCanonical::new(tikv_adapter.clone());
  let cache_service = prime_domain::CacheServiceCanonical::new(cache_repo);
  let entry_service = prime_domain::EntryServiceCanonical::new(entry_repo);
  let store_service = prime_domain::StoreServiceCanonical::new(store_repo);
  let token_service = prime_domain::TokenServiceCanonical::new(token_repo);

  let app_state = AppState {
    cache_service: Arc::new(Box::new(cache_service)),
    entry_service: Arc::new(Box::new(entry_service)),
    store_service: Arc::new(Box::new(store_service)),
    token_service: Arc::new(Box::new(token_service)),
  };

  let app = Router::new()
    .leptos_routes_with_context(
      &leptos_options,
      routes,
      {
        let app_state = app_state.clone();
        move || {
          provide_context(app_state.cache_service.clone());
          provide_context(app_state.entry_service.clone());
          provide_context(app_state.store_service.clone());
          provide_context(app_state.token_service.clone());
        }
      },
      {
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
      },
    )
    .fallback(leptos_axum::file_and_error_handler(shell))
    .layer(tower_http::compression::CompressionLayer::new())
    .with_state(leptos_options);

  log!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();

  Ok(())
}