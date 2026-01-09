//! App state for the grid service.

use std::{io::Read, path::PathBuf, sync::Arc};

use auth_domain::AuthDomainService;
use axum::extract::FromRef;
use domain::{
  DomainService, billing_domain::BillingService, db::Database,
  meta_domain::MetaService, mutate_domain::MutationService,
};
use metrics_domain::MetricsService;
use miette::{Context, IntoDiagnostic, Result};
use tower_sessions_db_store::DatabaseStore as DatabaseSessionStore;
use tracing::warn;

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

/// Configuration for serving HTTP responses.
#[derive(Debug)]
pub struct ServeConfig {
  /// The directory to serve static assets out of.
  pub static_asset_dir:   PathBuf,
  /// The stylesheet to be inlined into the doc `<head>`.
  pub inlined_stylesheet: Arc<str>,
  /// Whether to serve the non-minified version of the HTMX lib.
  pub non_minified_htmx:  bool,
}

impl ServeConfig {
  /// Build [`ServeConfig`] from the runtime environment.
  pub fn build() -> miette::Result<Self> {
    let static_asset_dir = std::env::var("GRID_STATIC_ASSET_DIR")
      .into_diagnostic()
      .context("`GRID_STATIC_ASSET_DIR` env var not populated")?;
    let static_asset_dir = PathBuf::from(static_asset_dir);

    let stylesheet_path = std::env::var("GRID_STYLESHEET_PATH")
      .into_diagnostic()
      .context("`GRID_STYLESHEET_PATH` env var not populated")?;
    let mut stylesheet_content = String::new();
    match std::fs::File::open(&stylesheet_path) {
      Ok(mut f) => {
        f.read_to_string(&mut stylesheet_content)
          .into_diagnostic()
          .context("failed to read from stylesheet file")?;
      }
      Err(e) => {
        tracing::warn!("failed to open stylesheet file: {e}");
      }
    };
    let stylesheet_content = Arc::<str>::from(stylesheet_content);

    let non_minified_htmx = std::env::var("GRID_NO_MINIFY_HTMX")
      .ok()
      .and_then(|val| match val.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" | "" => Some(false),
        v => {
          warn!(
            "did not recognize value for env var `GRID_NO_MINIFY_HTMX`, \
             ignoring: {v}"
          );
          None
        }
      })
      .unwrap_or(false);

    Ok(ServeConfig {
      static_asset_dir,
      inlined_stylesheet: stylesheet_content,
      non_minified_htmx,
    })
  }

  /// The name of the HTMX asset to serve.
  pub fn htmx_asset_name(&self) -> &'static str {
    match self.non_minified_htmx {
      true => "htmx.js",
      false => "htmx.min.js",
    }
  }
}

/// The state of a running grid service.
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
  /// The prime domain service.
  pub domain:         DomainService,
  /// The auth domain service.
  pub auth_domain:    AuthDomainService,
  /// The metrics domain service.
  pub metrics_domain: MetricsService,
  /// The user session store.
  pub session_store:  DatabaseSessionStore,
  /// The node metadata.
  pub node_meta:      Arc<NodeMeta>,
  /// The HTTP serving configuration.
  pub serve_config:   Arc<ServeConfig>,
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

    let node_meta =
      NodeMeta::from_env().context("failed to collect node metadata")?;
    let node_meta = Arc::new(node_meta);

    let serve_config =
      ServeConfig::build().context("failed to build serve config")?;
    let serve_config = Arc::new(serve_config);

    Ok(AppState {
      domain,
      auth_domain,
      metrics_domain,
      session_store,
      node_meta,
      serve_config,
    })
  }
}
