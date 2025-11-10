//! Provides [`MutationService`] for mutation operations on models.

mod create;
mod delete_entry;
mod patch_user;
mod user_active_org;

use db::Database;
use models::{Cache, Entry, Org, Store, User};

pub use self::user_active_org::UpdateActiveOrgError;

/// Service for mutation operations on models.
#[derive(Debug, Clone)]
pub struct MutationService {
  org_repo:   Database<Org>,
  user_repo:  Database<User>,
  store_repo: Database<Store>,
  entry_repo: Database<Entry>,
  cache_repo: Database<Cache>,
}

impl MutationService {
  /// Creates a new [`MutationService`].
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

  /// Creates a mocked-up [`MutationService`].
  pub fn new_mock() -> Self {
    Self {
      org_repo:   Database::new_mock(),
      user_repo:  Database::new_mock(),
      store_repo: Database::new_mock(),
      entry_repo: Database::new_mock(),
      cache_repo: Database::new_mock(),
    }
  }
}
