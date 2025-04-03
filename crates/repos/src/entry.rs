//! Provides a repository for the [`Entry`] domain model.

use std::sync::Arc;

use db::{FetchModelByIndexError, FetchModelError};
use hex::health::{self, HealthAware};
use models::{CacheRecordId, LaxSlug};
pub use models::{Entry, EntryCreateRequest};
use tracing::instrument;

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, Database};

/// The repository for the [`Entry`] domain model.
pub struct EntryRepository {
  inner: Arc<
    dyn ModelRepositoryLike<
      Model = Entry,
      ModelCreateRequest = Entry,
      CreateError = CreateModelError,
    >,
  >,
}

impl EntryRepository {
  /// Creates a new instance of the [`Entry`] repository using `BaseRepository`.
  pub fn new_from_base(db: Database<Entry>) -> Self {
    Self {
      inner: Arc::new(BaseRepository::new(db)),
    }
  }

  /// Creates a new model.
  #[instrument(skip(self))]
  pub async fn create_model(
    &self,
    input: EntryCreateRequest,
  ) -> Result<Entry, CreateModelError> {
    self.inner.create_model(input.into()).await
  }

  /// Fetches a model by its ID.
  #[instrument(skip(self))]
  pub async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Entry>,
  ) -> Result<Option<Entry>, FetchModelError> {
    self.inner.fetch_model_by_id(id).await
  }

  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's `INDICES` constant.
  #[instrument(skip(self))]
  pub async fn fetch_model_by_unique_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<Entry>, FetchModelByIndexError> {
    self
      .inner
      .fetch_model_by_unique_index(index_name, index_value)
      .await
  }

  /// Produces a list of all model IDs.
  #[instrument(skip(self))]
  pub async fn enumerate_models(&self) -> Result<Vec<Entry>> {
    self.inner.enumerate_models().await
  }

  /// Find an [`Entry`] by its cache ID and path.
  #[instrument(skip(self))]
  pub async fn find_entry_by_id_and_path(
    &self,
    cache_id: CacheRecordId,
    path: LaxSlug,
  ) -> Result<Option<Entry>, FetchModelByIndexError> {
    let index_value = LaxSlug::new(format!("{cache_id}-{path}"));
    self
      .fetch_model_by_unique_index("cache-id-path".into(), index_value.into())
      .await
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for EntryRepository {
  fn name(&self) -> &'static str { stringify!(EntryRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}
