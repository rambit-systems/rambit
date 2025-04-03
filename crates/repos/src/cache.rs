//! Provides a repository for the [`Cache`] domain model.

use std::sync::Arc;

use db::{FetchModelByIndexError, FetchModelError};
use hex::health::{self, HealthAware};
use models::StrictSlug;
pub use models::{Cache, CacheCreateRequest};
use tracing::instrument;

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, Database};

/// The repository for the [`Cache`] domain model.
pub struct CacheRepository {
  inner: Arc<
    dyn ModelRepositoryLike<
      Model = Cache,
      ModelCreateRequest = Cache,
      CreateError = CreateModelError,
    >,
  >,
}

impl CacheRepository {
  /// Creates a new instance of the [`Cache`] repository using `BaseRepository`.
  pub fn new_from_base(db: Database<Cache>) -> Self {
    Self {
      inner: Arc::new(BaseRepository::new(db)),
    }
  }

  /// Creates a new model.
  #[instrument(skip(self))]
  pub async fn create_model(
    &self,
    input: CacheCreateRequest,
  ) -> Result<Cache, CreateModelError> {
    self.inner.create_model(input.into()).await
  }

  /// Fetches a model by its ID.
  #[instrument(skip(self))]
  pub async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Cache>,
  ) -> Result<Option<Cache>, FetchModelError> {
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
  ) -> Result<Option<Cache>, FetchModelByIndexError> {
    self
      .inner
      .fetch_model_by_unique_index(index_name, index_value)
      .await
  }

  /// Produces a list of all model IDs.
  #[instrument(skip(self))]
  pub async fn enumerate_models(&self) -> Result<Vec<Cache>> {
    self.inner.enumerate_models().await
  }

  /// Find a [`Cache`] by its name.
  #[instrument(skip(self))]
  pub async fn find_by_name(
    &self,
    name: StrictSlug,
  ) -> Result<Option<Cache>, FetchModelByIndexError> {
    self
      .fetch_model_by_unique_index("name".to_string(), EitherSlug::Strict(name))
      .await
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for CacheRepository {
  fn name(&self) -> &'static str { stringify!(CacheRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}
