//! Applies migrations to the database.

use std::time::Duration;

use db::Migrator;
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

  let org_db = db::Database::new_from_kv(retryable_kv_store.clone());
  let user_db = db::Database::new_from_kv(retryable_kv_store.clone());
  let store_db = db::Database::new_from_kv(retryable_kv_store.clone());
  let cache_db = db::Database::new_from_kv(retryable_kv_store.clone());
  let token_db = db::Database::new_from_kv(retryable_kv_store.clone());

  let migrator = Migrator::new(org_db, user_db, store_db, cache_db, token_db);
  migrator.migrate().await?;

  tokio::time::sleep(std::time::Duration::from_secs(1)).await;

  Ok(())
}
