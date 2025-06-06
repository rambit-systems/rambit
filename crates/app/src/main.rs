//! The server-side entrypoint for Rambit.

use miette::Result;
use prime_domain::{
  PrimeDomainService,
  db::{Database, kv},
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(env_filter)
    .init();

  tracing::info!("starting app server");

  let kv_store_location = std::path::PathBuf::from(
    std::env::var("REDB_STORE_PATH").unwrap_or("/tmp/picturepro-db".to_owned()),
  );
  let kv_store = kv::KeyValueStore::new_redb(&kv_store_location)?;

  let org_db = Database::new_from_kv(kv_store.clone());
  let user_db = Database::new_from_kv(kv_store.clone());
  let store_db = Database::new_from_kv(kv_store.clone());
  let entry_db = Database::new_from_kv(kv_store.clone());
  let cache_db = Database::new_from_kv(kv_store);

  let prime_domain_service =
    PrimeDomainService::new(org_db, user_db, store_db, entry_db, cache_db);

  prime_domain_service.migrate_test_data().await;

  Ok(())
}
