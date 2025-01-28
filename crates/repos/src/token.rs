//! Provides a repository for the [`Token`] domain model.

use std::sync::Arc;

use hex::health::{self, HealthAware};
pub use models::{Token, TokenCreateRequest};
use tracing::instrument;

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, Database};

/// The repository for the [`Token`] domain model.
pub struct TokenRepository {
  inner: Arc<
    dyn ModelRepositoryLike<
      Model = Token,
      ModelCreateRequest = Token,
      CreateError = CreateModelError,
    >,
  >,
}

impl TokenRepository {
  /// Creates a new instance of the [`Token`] repository using `BaseRepository`.
  pub fn new_from_base(db: Database<Token>) -> Self {
    Self {
      inner: Arc::new(BaseRepository::new(db)),
    }
  }

  /// Creates a new model.
  #[instrument(skip(self))]
  async fn create_model(
    &self,
    input: TokenCreateRequest,
  ) -> Result<Token, CreateModelError> {
    self.inner.create_model(input.into()).await
  }

  /// Fetches a model by its ID.
  #[instrument(skip(self))]
  async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Token>,
  ) -> Result<Option<Token>, FetchModelError> {
    self.inner.fetch_model_by_id(id).await
  }

  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's `INDICES` constant.
  #[instrument(skip(self))]
  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<Token>, FetchModelByIndexError> {
    self
      .inner
      .fetch_model_by_index(index_name, index_value)
      .await
  }

  /// Produces a list of all model IDs.
  #[instrument(skip(self))]
  async fn enumerate_models(&self) -> Result<Vec<Token>> {
    self.inner.enumerate_models().await
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for TokenRepository {
  fn name(&self) -> &'static str { stringify!(TokenRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}
