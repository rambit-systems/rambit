pub use db::CreateModelError;
pub(crate) use db::{Database, FetchModelByIndexError, FetchModelError};
use hex::health::{self, HealthAware};
use miette::Result;
use tracing::instrument;

use crate::ModelRepository;

/// Provides a base repository implementation for any model.
///
/// This is private and cannot be used directly. Each model's implementation
/// of `ModelRepository` needs to be a concrete type, even if it's just a
/// shell for this type, so that extra logic can be added later if needed.
pub(crate) struct BaseRepository<M: models::Model> {
  db: Database<M>,
}

impl<M: models::Model> Clone for BaseRepository<M> {
  fn clone(&self) -> Self {
    Self {
      db: self.db.clone(),
    }
  }
}

impl<M: models::Model> BaseRepository<M> {
  pub fn new(db: Database<M>) -> Self {
    tracing::info!(
      "creating new `BaseRepository<{:?}>` instance",
      M::TABLE_NAME
    );

    Self { db }
  }
}

#[async_trait::async_trait]
impl<M: models::Model> health::HealthReporter for BaseRepository<M> {
  fn name(&self) -> &'static str { stringify!(BaseRepository<M>) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(self.db.health_report()))
      .await
      .into()
  }
}

#[async_trait::async_trait]
impl<M: models::Model> ModelRepository for BaseRepository<M> {
  type Model = M;
  type ModelCreateRequest = M;
  type CreateError = CreateModelError;

  #[instrument(skip(self))]
  async fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> Result<Self::Model, CreateModelError> {
    self.db.create_model(input).await
  }

  #[instrument(skip(self))]
  async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> Result<Option<Self::Model>, FetchModelError> {
    self.db.fetch_model_by_id(id).await
  }

  #[instrument(skip(self))]
  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: models::EitherSlug,
  ) -> Result<Option<Self::Model>, FetchModelByIndexError> {
    self.db.fetch_model_by_index(index_name, index_value).await
  }

  #[instrument(skip(self))]
  async fn enumerate_models(&self) -> Result<Vec<Self::Model>> {
    self.db.enumerate_models().await
  }
}
