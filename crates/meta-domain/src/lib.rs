//! Provides [`MetaService`] for read-only only operations on models.

mod entry_counts;
mod fetch_by_id;
mod fetch_by_name;
mod fetch_by_org;
mod fetch_entry_by;
mod fetch_user_by;
mod search_stores_by_user;

use db::Database;
use models::{Cache, Entry, Org, Store, User};

pub use self::search_stores_by_user::SearchByUserError;

/// Service for read-only operations on models.
#[derive(Debug, Clone)]
pub struct MetaService {
  org_repo:   Database<Org>,
  user_repo:  Database<User>,
  store_repo: Database<Store>,
  entry_repo: Database<Entry>,
  cache_repo: Database<Cache>,
}

impl MetaService {
  /// Creates a new [`MetaService`].
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

  /// Creates a mocked-up [`MetaService`].
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
