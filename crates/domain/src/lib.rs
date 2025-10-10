//! Entrypoint for domain logic.

mod create;
mod delete_entry;
pub mod download;
mod migrate;
pub mod mutate_user;
pub mod narinfo;
pub mod upload;

pub use belt;
pub use bytes;
pub use db;
use db::Database;
use meta_domain::MetaService;
pub use models;
use models::{Cache, Entry, Org, Store, User};

/// The domain service type.
#[derive(Debug, Clone)]
pub struct DomainService {
  meta:       MetaService,
  org_repo:   Database<Org>,
  user_repo:  Database<User>,
  store_repo: Database<Store>,
  entry_repo: Database<Entry>,
  cache_repo: Database<Cache>,
}

impl DomainService {
  /// Create a new [`DomainService`].
  pub fn new(
    org_repo: Database<Org>,
    user_repo: Database<User>,
    store_repo: Database<Store>,
    entry_repo: Database<Entry>,
    cache_repo: Database<Cache>,
  ) -> Self {
    let meta = MetaService::new(
      org_repo.clone(),
      user_repo.clone(),
      store_repo.clone(),
      entry_repo.clone(),
      cache_repo.clone(),
    );

    Self {
      meta,
      org_repo,
      user_repo,
      store_repo,
      entry_repo,
      cache_repo,
    }
  }

  /// Access the internal [`MetaService`].
  pub fn meta(&self) -> &MetaService { &self.meta }
}

#[cfg(test)]
mod tests {
  use db::Database;

  use crate::DomainService;

  impl DomainService {
    pub(crate) async fn mock_domain() -> DomainService {
      let pds = DomainService::new(
        Database::new_mock(),
        Database::new_mock(),
        Database::new_mock(),
        Database::new_mock(),
        Database::new_mock(),
      );

      pds
        .migrate_test_data(true)
        .await
        .expect("failed to migrate test data");

      pds
    }
  }
}
