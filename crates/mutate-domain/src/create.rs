use db::DatabaseError;
use models::{Cache, Entry, Org, RecordId, Store, User};

use super::MutationService;

impl MutationService {
  /// Creates a [`Cache`].
  #[tracing::instrument(skip(self))]
  pub async fn create_cache(
    &self,
    cache: &Cache,
  ) -> Result<RecordId<Cache>, DatabaseError> {
    self.cache_repo.insert(cache).await.map(|()| cache.id)
  }

  /// Creates a [`Store`].
  #[tracing::instrument(skip(self))]
  pub async fn create_store(
    &self,
    store: &Store,
  ) -> Result<RecordId<Store>, DatabaseError> {
    self.store_repo.insert(store).await.map(|()| store.id)
  }

  /// Creates an [`Org`].
  #[tracing::instrument(skip(self))]
  pub async fn create_org(
    &self,
    org: &Org,
  ) -> Result<RecordId<Org>, DatabaseError> {
    self.org_repo.insert(org).await.map(|()| org.id)
  }

  /// Creates a [`User`].
  pub async fn create_user(
    &self,
    user: &User,
  ) -> Result<RecordId<User>, DatabaseError> {
    self.user_repo.insert(user).await.map(|()| user.id)
  }

  /// Creates an [`Entry`].
  #[tracing::instrument(skip(self))]
  pub async fn create_entry(
    &self,
    entry: &Entry,
  ) -> Result<RecordId<Entry>, DatabaseError> {
    self.entry_repo.insert(entry).await.map(|()| entry.id)
  }
}
