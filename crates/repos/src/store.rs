//! Provides a repository for the [`Store`] domain model.

use std::sync::Arc;

use hex::health::{self, HealthAware};
pub use models::{Store, StoreCreateRequest};
use tracing::instrument;

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, Database};

/// The repository for the [`Store`] domain model.
pub struct StoreRepository {
  inner: Arc<
    dyn ModelRepositoryLike<
      Model = Store,
      ModelCreateRequest = Store,
      CreateError = CreateModelError,
    >,
  >,
}

impl StoreRepository {
  /// Creates a new instance of the [`Store`] repository using `BaseRepository`.
  pub fn new_from_base(db: Database<Store>) -> Self {
    Self {
      inner: Arc::new(BaseRepository::new(db)),
    }
  }

  /// Creates a new model.
  #[instrument(skip(self))]
  pub async fn create_model(
    &self,
    input: StoreCreateRequest,
  ) -> Result<Store, CreateModelError> {
    self.inner.create_model(input.into()).await
  }

  /// Fetches a model by its ID.
  #[instrument(skip(self))]
  pub async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Store>,
  ) -> Result<Option<Store>, FetchModelError> {
    self.inner.fetch_model_by_id(id).await
  }

  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's `INDICES` constant.
  #[instrument(skip(self))]
  pub async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<Store>, FetchModelByIndexError> {
    self
      .inner
      .fetch_model_by_index(index_name, index_value)
      .await
  }

  /// Produces a list of all model IDs.
  #[instrument(skip(self))]
  pub async fn enumerate_models(&self) -> Result<Vec<Store>> {
    self.inner.enumerate_models().await
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for StoreRepository {
  fn name(&self) -> &'static str { stringify!(StoreRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}
