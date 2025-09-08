//! Entrypoint for domain logic.

mod counts;
mod create;
pub mod download;
mod fetch_by_id;
mod fetch_by_name;
mod fetch_by_org;
mod migrate;
pub mod narinfo;
mod search_by_user;
pub mod upload;

pub use belt;
pub use bytes;
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

#[cfg(test)]
mod tests {
  use db::Database;

  use crate::PrimeDomainService;

  impl PrimeDomainService {
    pub(crate) async fn mock_prime_domain() -> PrimeDomainService {
      let pds = PrimeDomainService {
        org_repo:   Database::new_mock(),
        user_repo:  Database::new_mock(),
        store_repo: Database::new_mock(),
        entry_repo: Database::new_mock(),
        cache_repo: Database::new_mock(),
      };

      pds
        .migrate_test_data(true)
        .await
        .expect("failed to migrate test data");

      pds
    }
  }
}
