//! App state for the grid service.

use std::sync::Arc;

use auth_domain::AuthDomainService;
use axum::extract::FromRef;
use domain::{
  DomainService, billing_domain::BillingService, db::Database,
  meta_domain::MetaService, mutate_domain::MutationService,
};
use leptos::config::LeptosOptions;
use metrics_domain::MetricsService;
use miette::{Context, IntoDiagnostic, Result};
use tower_sessions_db_store::DatabaseStore as DatabaseSessionStore;

/// Metadata for a node serving the grid service.
#[derive(Debug)]
pub struct NodeMeta {
  /// A string descriptor of the node's environment (dev, staging, prod, etc.).
  pub environment: String,
  /// The name of the node host.
  pub host_name:   String,
}

impl NodeMeta {
  /// Collect [`NodeMeta`] from the runtime environment.
  pub fn from_env() -> miette::Result<Self> {
    let env = std::env::var("GRID_ENV")
      .into_diagnostic()
      .context("`GRID_ENV` env var not populated")?;

    let host_name = gethostname::gethostname()
      .into_string()
      .map_err(|_| miette::miette!("hostname was not unicode"))?;

    Ok(NodeMeta {
      environment: env,
      host_name,
    })
  }
}

/// The state of a running grid service.
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
  /// The auth domain service.
  pub auth_domain:    AuthDomainService,
  /// The prime domain service.
  pub domain:         DomainService,
  /// The metrics domain service.
  pub metrics_domain: MetricsService,
  /// The user session store.
  pub session_store:  DatabaseSessionStore,
  /// Options for leptos.
  pub leptos_options: LeptosOptions,
  /// The node metadata.
  pub node_meta:      Arc<NodeMeta>,
}

impl AppState {
  /// Builds the [`AppState`].
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

    org_db.initialize_schema().await?;
    user_db.initialize_schema().await?;
    store_db.initialize_schema().await?;
    entry_db.initialize_schema().await?;
    cache_db.initialize_schema().await?;
    session_db.initialize_schema().await?;

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
    let billing_domain = BillingService::new_from_env()
      .context("failed to create BillingService")?;
    let metrics_domain = MetricsService::new_from_env()
      .context("failed to create MetricService")?;

    let domain = DomainService::new(meta_domain, mutate_domain, billing_domain);
    let auth_domain = AuthDomainService::new(domain.clone());
    let session_store = DatabaseSessionStore::new(session_db);

    let leptos_conf = leptos::prelude::get_configuration(None)
      .into_diagnostic()
      .context("failed to prepare leptos config")?;
    let leptos_options = leptos_conf.leptos_options;

    let node_meta =
      NodeMeta::from_env().context("failed to collect node metadata")?;
    let node_meta = Arc::new(node_meta);

    Ok(AppState {
      auth_domain,
      domain,
      metrics_domain,
      session_store,
      leptos_options,
      node_meta,
    })
  }
}
