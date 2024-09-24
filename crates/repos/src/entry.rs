//! Provides a repository for the [`Entry`] domain model.

use models::{CacheRecordId, LaxSlug};
pub use models::{Entry, EntryCreateRequest};

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, DatabaseAdapter};

/// Descriptor trait for repositories that handle [`Entry`] domain model.
pub trait EntryRepository:
  ModelRepository<
  Model = Entry,
  ModelCreateRequest = EntryCreateRequest,
  CreateError = CreateModelError,
>
{
  /// Find an [`Entry`] by its cache ID and path.
  fn find_by_entry_id_and_path(
    &self,
    cache_id: CacheRecordId,
    path: LaxSlug,
  ) -> impl Future<Output = Result<Option<Entry>, FetchModelByIndexError>> + Send;
}

impl<T> EntryRepository for T
where
  T: ModelRepository<
    Model = Entry,
    ModelCreateRequest = EntryCreateRequest,
    CreateError = CreateModelError,
  >,
{
  /// Find an [`Entry`] by its cache ID and path.
  async fn find_by_entry_id_and_path(
    &self,
    cache_id: CacheRecordId,
    path: LaxSlug,
  ) -> Result<Option<Entry>, FetchModelByIndexError> {
    let index_value = LaxSlug::new(format!("{cache_id}-{path}"));
    self
      .fetch_model_by_index("cache-id-path".into(), index_value.into())
      .await
  }
}

/// The repository for the [`Entry`] domain model.
pub struct EntryRepositoryCanonical<DB: DatabaseAdapter> {
  base_repo: BaseRepository<Entry, DB>,
}

impl<DB: DatabaseAdapter> Clone for EntryRepositoryCanonical<DB> {
  fn clone(&self) -> Self {
    Self {
      base_repo: self.base_repo.clone(),
    }
  }
}

impl<DB: DatabaseAdapter> EntryRepositoryCanonical<DB> {
  /// Create a new instance of the [`Entry`] repository.
  pub fn new(db_adapter: DB) -> Self {
    Self {
      base_repo: BaseRepository::new(db_adapter),
    }
  }
}

impl<DB: DatabaseAdapter> ModelRepository for EntryRepositoryCanonical<DB> {
  type Model = Entry;
  type ModelCreateRequest = EntryCreateRequest;
  type CreateError = CreateModelError;

  fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> impl Future<Output = Result<(), Self::CreateError>> + Send {
    self.base_repo.create_model(input.into())
  }

  fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> impl Future<Output = Result<Option<Self::Model>, FetchModelError>> + Send
  {
    self.base_repo.fetch_model_by_id(id)
  }

  fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> impl Future<Output = Result<Option<Self::Model>, FetchModelByIndexError>> + Send
  {
    self.base_repo.fetch_model_by_index(index_name, index_value)
  }
}