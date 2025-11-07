use auth_domain::AuthDomainService;
use axum::extract::FromRef;
use domain::{
  DomainService, db::Database, meta_domain::MetaService,
  mutate_domain::MutationService,
};
use leptos::config::LeptosOptions;
use miette::{Context, IntoDiagnostic, Result};
use tower_sessions_db_store::DatabaseStore as DatabaseSessionStore;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
  pub auth_domain:    AuthDomainService,
  pub domain:         DomainService,
  pub session_store:  DatabaseSessionStore,
  pub leptos_options: LeptosOptions,
}

impl AppState {
  pub async fn build() -> Result<Self> {
    let (org_db, user_db, store_db, entry_db, cache_db, session_db) = {
      let url = std::env::var("POSTGRES_URL")
        .into_diagnostic()
        .context("`POSTGRES_URL` env var not populated")?;
      let pool = db::PgPool::connect(&url)
        .await
        .into_diagnostic()
        .context("failed to connect to postgres")?;

      (
        Database::new_postgres_from_pool(pool.clone()),
        Database::new_postgres_from_pool(pool.clone()),
        Database::new_postgres_from_pool(pool.clone()),
        Database::new_postgres_from_pool(pool.clone()),
        Database::new_postgres_from_pool(pool.clone()),
        Database::new_postgres_from_pool(pool),
      )
    };

    let meta_domain = MetaService::new(
      org_db.clone(),
      user_db.clone(),
      store_db.clone(),
      entry_db.clone(),
      cache_db.clone(),
    );
    let mutate_domain = MutationService::new(
      org_db.clone(),
      user_db.clone(),
      store_db.clone(),
      entry_db.clone(),
      cache_db,
    );

    let auth_domain =
      AuthDomainService::new(meta_domain.clone(), mutate_domain.clone());
    let domain = DomainService::new(meta_domain, mutate_domain);
    let session_store = DatabaseSessionStore::new(session_db);

    let leptos_conf = leptos::prelude::get_configuration(None)
      .into_diagnostic()
      .context("failed to prepare leptos config")?;
    let leptos_options = leptos_conf.leptos_options;

    Ok(AppState {
      auth_domain,
      domain,
      session_store,
      leptos_options,
    })
  }
}
