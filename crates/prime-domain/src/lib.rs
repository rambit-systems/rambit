//! Entrypoint for domain logic.

mod fetch_by_id;
mod migrate;
mod upload;

pub use db;
use db::Database;
pub use models;
use models::{Cache, Entry, Org, Store, User};

/// The prime domain service type.
#[derive(Debug, Clone)]
pub struct PrimeDomainService {
  org_repo:   Database<Org>,
  user_repo:  Database<User>,
  store_repo: Database<Store>,
  entry_repo: Database<Entry>,
  cache_repo: Database<Cache>,
}

impl PrimeDomainService {
  /// Create a new [`PrimeDomainService`].
  pub fn new(
    org_repo: Database<Org>,
    user_repo: Database<User>,
    store_repo: Database<Store>,
    entry_repo: Database<Entry>,
    cache_repo: Database<Cache>,
  ) -> Self {
    Self {
      org_repo,
      user_repo,
      store_repo,
      entry_repo,
      cache_repo,
    }
  }
}
