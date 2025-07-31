use auth_domain::AuthDomainService;
use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use miette::{Context, IntoDiagnostic, Result};
use prime_domain::{PrimeDomainService, db::Database};
use tower_sessions_db_store::DatabaseStore as DatabaseSessionStore;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
  pub auth_domain:    AuthDomainService,
  pub prime_domain:   PrimeDomainService,
  pub session_store:  DatabaseSessionStore,
  pub leptos_options: LeptosOptions,
}

impl AppState {
  pub async fn build() -> Result<Self> {
    let kv_store_location = std::path::PathBuf::from(
      std::env::var("REDB_STORE_PATH").unwrap_or("/tmp/rambit-db".to_owned()),
    );
    let kv_store = kv::KeyValueStore::new_redb(&kv_store_location)?;

    let org_db = Database::new_from_kv(kv_store.clone());
    let user_db = Database::new_from_kv(kv_store.clone());
    let store_db = Database::new_from_kv(kv_store.clone());
    let entry_db = Database::new_from_kv(kv_store.clone());
    let cache_db = Database::new_from_kv(kv_store.clone());
    let session_db = Database::new_from_kv(kv_store);

    let leptos_conf = leptos::prelude::get_configuration(None)
      .into_diagnostic()
      .context("failed to prepare leptos config")?;

    Ok(AppState {
      auth_domain:    AuthDomainService::new(user_db.clone()),
      prime_domain:   PrimeDomainService::new(
        org_db, user_db, store_db, entry_db, cache_db,
      ),
      session_store:  DatabaseSessionStore::new(session_db),
      leptos_options: leptos_conf.leptos_options,
    })
  }
}
