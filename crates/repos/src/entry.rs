//! Provides a repository for the [`Entry`] domain model.

use db::{FetchModelByIndexError, FetchModelError};
use hex::health::{self, HealthAware};
use models::{CacheRecordId, LaxSlug};
pub use models::{Entry, EntryCreateRequest};
use tracing::instrument;

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, Database};

/// Descriptor trait for repositories that handle [`Entry`] domain model.
#[async_trait::async_trait]
pub trait EntryRepository:
  ModelRepositoryLike<
  Model = Entry,
  ModelCreateRequest = EntryCreateRequest,
  CreateError = CreateModelError,
>
{
  /// Find an [`Entry`] by its cache ID and path.
  #[instrument(skip(self))]
  async fn find_entry_by_id_and_path(
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

impl<T> EntryRepository for T where
  T: ModelRepositoryLike<
    Model = Entry,
    ModelCreateRequest = EntryCreateRequest,
    CreateError = CreateModelError,
  >
{
}

/// The repository for the [`Entry`] domain model.
#[derive(Clone)]
pub struct EntryRepositoryCanonical {
  base_repo: BaseRepository<Entry>,
}

impl EntryRepositoryCanonical {
  /// Create a new instance of the [`Entry`] repository.
  pub fn new(db: Database<Entry>) -> Self {
    tracing::info!("creating new `EntryRepositoryCanonical` instance");
    Self {
      base_repo: BaseRepository::new(db),
    }
  }
}

crate::impl_repository_on_base!(
  EntryRepositoryCanonical,
  Entry,
  EntryCreateRequest,
  CreateModelError
);
