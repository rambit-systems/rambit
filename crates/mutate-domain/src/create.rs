use db::CreateModelError;
use models::{
  Cache, Entry, Org, StorageCredentials, Store, StoreConfiguration, User,
  dvf::{EntityName, RecordId, Visibility},
};

use super::MutationService;

impl MutationService {
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

  /// Creates an [`Org`].
  pub async fn create_org(
    &self,
    org: Org,
  ) -> Result<RecordId<Org>, CreateModelError> {
    self.org_repo.create_model(org).await.map(|s| s.id)
  }

  /// Creates a [`User`].
  pub async fn create_user(
    &self,
    user: User,
  ) -> Result<RecordId<User>, CreateModelError> {
    self.user_repo.create_model(user).await.map(|u| u.id)
  }

  /// Creates an [`Entry`].
  pub async fn create_entry(
    &self,
    entry: Entry,
  ) -> Result<RecordId<Entry>, CreateModelError> {
    self.entry_repo.create_model(entry).await.map(|s| s.id)
  }
}
