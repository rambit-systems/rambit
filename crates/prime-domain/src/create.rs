use db::CreateModelError;
use models::{
  Cache, Org, StorageCredentials, Store, StoreConfiguration,
  dvf::{EntityName, RecordId, Visibility},
};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Creates a [`Cache`].
  pub async fn create_cache(
    &self,
    org: RecordId<Org>,
    name: EntityName,
    visibility: Visibility,
  ) -> Result<RecordId<Cache>, CreateModelError> {
    self
      .cache_repo
      .create_model(Cache {
        id: RecordId::new(),
        org,
        name,
        visibility,
      })
      .await
      .map(|c| c.id)
  }

  /// Creates a [`Store`].
  pub async fn create_store(
    &self,
    org: RecordId<Org>,
    name: EntityName,
    credentials: StorageCredentials,
    config: StoreConfiguration,
  ) -> Result<RecordId<Store>, CreateModelError> {
    self
      .store_repo
      .create_model(Store {
        id: RecordId::new(),
        org,
        credentials,
        config,
        name,
      })
      .await
      .map(|s| s.id)
  }
}
