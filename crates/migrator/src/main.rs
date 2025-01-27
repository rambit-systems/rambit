//! Applies migrations to the database.

use std::{sync::Arc, time::Duration};

use db::Migratable;
use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let filter = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or(tracing_subscriber::EnvFilter::new("info"));
  tracing_subscriber::fmt().with_env_filter(filter).init();

  let retryable_kv_store = db::kv::KeyValueStore::new_retryable_tikv_from_env(
    5,
    Duration::from_secs(2),
  )
  .await;
  let db = Arc::new(db::KvDatabaseAdapter::new(retryable_kv_store));
  db.migrate().await?;

  tokio::time::sleep(std::time::Duration::from_secs(1)).await;

  Ok(())
}
