use db::DatabaseError;
use models::{Cache, Org, RecordId, Store};

use crate::DomainService;

impl DomainService {
  /// Creates a [`Cache`].
  #[tracing::instrument(skip(self))]
  pub async fn create_cache(
    &self,
    cache: &Cache,
  ) -> Result<RecordId<Cache>, DatabaseError> {
    self.mutate.create_cache(cache).await
  }

  /// Creates a [`Store`].
  #[tracing::instrument(skip(self))]
  pub async fn create_store(
    &self,
    store: &Store,
  ) -> Result<RecordId<Store>, DatabaseError> {
    self.mutate.create_store(store).await
  }

  /// Creates an [`Org`].
  #[tracing::instrument(skip(self))]
  pub async fn create_org(
    &self,
    org: &Org,
  ) -> Result<RecordId<Org>, DatabaseError> {
    self.mutate.create_org(org).await
  }
}
