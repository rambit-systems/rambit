use db::CreateModelError;
use models::{
  Cache, Org, StorageCredentials, Store, StoreConfiguration,
  dvf::{EntityName, RecordId, Visibility},
};

use crate::DomainService;

impl DomainService {
  /// Creates a [`Cache`].
  pub async fn create_cache(
    &self,
    org: RecordId<Org>,
    name: EntityName,
    visibility: Visibility,
  ) -> Result<RecordId<Cache>, CreateModelError> {
    self.mutate.create_cache(org, name, visibility).await
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
      .mutate
      .create_store(org, name, credentials, config)
      .await
  }

  /// Creates an [`Org`].
  pub async fn create_org(
    &self,
    name: EntityName,
  ) -> Result<RecordId<Org>, CreateModelError> {
    self
      .mutate
      .create_org(Org {
        id:        RecordId::new(),
        org_ident: models::OrgIdent::Named(name),
      })
      .await
  }
}
