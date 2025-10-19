//! Provides [`MutationService`] for mutation operations on models.

mod create;
mod delete_entry;
mod migrate;
mod patch_user;

use db::Database;
use models::{Cache, Entry, Org, Store, User};

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
}
