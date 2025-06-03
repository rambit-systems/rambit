use db::FetchModelError;
use models::{Cache, Entry, Org, Store, User, dvf::RecordId};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Fetches an [`Org`] by its ID.
  pub async fn fetch_org_by_id(
    &self,
    id: RecordId<Org>,
  ) -> Result<Option<Org>, FetchModelError> {
    self.org_repo.fetch_model_by_id(id).await
  }

  /// Fetches a [`User`] by its ID.
  pub async fn fetch_user_by_id(
    &self,
    id: RecordId<User>,
  ) -> Result<Option<User>, FetchModelError> {
    self.user_repo.fetch_model_by_id(id).await
  }

  /// Fetches an [`Store`] by its ID.
  pub async fn fetch_store_by_id(
    &self,
    id: RecordId<Store>,
  ) -> Result<Option<Store>, FetchModelError> {
    self.store_repo.fetch_model_by_id(id).await
  }

  /// Fetches an [`Entry`] by its ID.
  pub async fn fetch_entry_by_id(
    &self,
    id: RecordId<Entry>,
  ) -> Result<Option<Entry>, FetchModelError> {
    self.entry_repo.fetch_model_by_id(id).await
  }

  /// Fetches an [`Cache`] by its ID.
  pub async fn fetch_cache_by_id(
    &self,
    id: RecordId<Cache>,
  ) -> Result<Option<Cache>, FetchModelError> {
    self.cache_repo.fetch_model_by_id(id).await
  }
}
