//! The leptos server crate for the Cartographer app.

use std::{sync::Arc, time::Duration};

use axum::{extract::FromRef, Router};
use cart_app::*;
use leptos::{logging::log, prelude::*};
use leptos_axum::{generate_route_list, LeptosRoutes};
use prime_domain::{repos::db::Database, DynPrimeDomainService};

#[derive(Clone, FromRef)]
struct AppState {
  prime_domain_service: DynPrimeDomainService,
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

  let retryable_kv_store =
    prime_domain::repos::db::kv::KeyValueStore::new_retryable_tikv_from_env(
      5,
      Duration::from_secs(2),
    )
    .await;
  let cache_repo = prime_domain::repos::CacheRepository::new_from_base(
    Database::new_from_kv(retryable_kv_store.clone()),
  );
  let entry_repo = prime_domain::repos::EntryRepository::new_from_base(
    Database::new_from_kv(retryable_kv_store.clone()),
  );
  let store_repo = prime_domain::repos::StoreRepository::new_from_base(
    Database::new_from_kv(retryable_kv_store.clone()),
  );
  let token_repo = prime_domain::repos::TokenRepository::new_from_base(
    Database::new_from_kv(retryable_kv_store.clone()),
  );
  let temp_storage_repo =
    prime_domain::repos::TempStorageRepository::new_from_mock(
      std::path::PathBuf::from("/tmp/rambit-temp-storage"),
    );
  let user_storage_repo = prime_domain::repos::UserStorageRepository::new();
  let prime_domain_service = prime_domain::PrimeDomainServiceCanonical::new(
    cache_repo,
    entry_repo,
    store_repo,
    token_repo,
    temp_storage_repo,
    user_storage_repo,
  );

  let app_state = AppState {
    prime_domain_service: Arc::new(Box::new(prime_domain_service)),
  };

  let app = Router::new()
    .leptos_routes_with_context(
      &leptos_options,
      routes,
      {
        let app_state = app_state.clone();
        move || {
          provide_context(app_state.prime_domain_service.clone());
        }
      },
      {
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
      },
    )
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(leptos_options);

  log!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();

  Ok(())
}
