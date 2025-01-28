//! Provides repository traits and implementors.
//!
//! Repositories are used to abstract the storage layer from the rest of the
//! application. They are used by services to interact with table-like or object
//! storage.

mod base;
mod cache;
mod entry;
mod store;
mod temp_storage;
mod token;
mod user_storage;

pub use db;
use db::{FetchModelByIndexError, FetchModelError};
use hex::Hexagonal;
use miette::Result;
use models::EitherSlug;
pub use storage::{
  belt,
  temp::{TempStorageCreds, TempStorageCredsError},
};

pub use self::{
  cache::*, entry::*, store::*, temp_storage::*, token::*, user_storage::*,
};

/// Defines a repository interface for models.
#[async_trait::async_trait]
pub(crate) trait ModelRepositoryLike: Hexagonal {
  /// The model type.
  type Model: models::Model;
  /// The request type for creating a model.
  type ModelCreateRequest: std::fmt::Debug + Send + Sync + 'static;
  /// The error type for creating a model.
  type CreateError: std::error::Error + Send + Sync + 'static;

  /// Creates a new model.
  async fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> Result<Self::Model, Self::CreateError>;

  /// Fetches a model by its ID.
  async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> Result<Option<Self::Model>, FetchModelError>;

  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's `INDICES` constant.
  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<Self::Model>, FetchModelByIndexError>;

  /// Produces a list of all model IDs.
  async fn enumerate_models(&self) -> Result<Vec<Self::Model>>;
}

#[async_trait::async_trait]
impl<T, I> ModelRepositoryLike for T
where
  T: std::ops::Deref<Target = I> + Hexagonal + Sized,
  I: ModelRepositoryLike + ?Sized,
{
  type Model = I::Model;
  type ModelCreateRequest = I::ModelCreateRequest;
  type CreateError = I::CreateError;

  async fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> Result<Self::Model, Self::CreateError> {
    I::create_model(self, input).await
  }
  async fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> Result<Option<Self::Model>, FetchModelError> {
    I::fetch_model_by_id(self, id).await
  }
  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<Self::Model>, FetchModelByIndexError> {
    I::fetch_model_by_index(self, index_name, index_value).await
  }
  async fn enumerate_models(&self) -> Result<Vec<Self::Model>> {
    I::enumerate_models(self).await
  }
}

/// Macro to implement the `ModelRepository` trait for a given repository.
#[macro_export]
macro_rules! impl_repository_on_base {
  ($canonical:ident, $model:ty, $create_request:ty, $create_error:ty) => {
    #[async_trait::async_trait]
    impl health::HealthReporter for $canonical {
      fn name(&self) -> &'static str { stringify!($canonical<DB>) }
      async fn health_check(&self) -> health::ComponentHealth {
        health::AdditiveComponentHealth::from_futures(Some(
          self.base_repo.health_report(),
        ))
        .await
        .into()
      }
    }

    #[async_trait::async_trait]
    impl ModelRepositoryLike for $canonical {
      type Model = $model;
      type ModelCreateRequest = $create_request;
      type CreateError = $create_error;

      #[instrument(skip(self))]
      async fn create_model(
        &self,
        input: Self::ModelCreateRequest,
      ) -> Result<Self::Model, Self::CreateError> {
        self.base_repo.create_model(input.into()).await
      }

      #[instrument(skip(self))]
      async fn fetch_model_by_id(
        &self,
        id: models::RecordId<Self::Model>,
      ) -> Result<Option<Self::Model>, FetchModelError> {
        self.base_repo.fetch_model_by_id(id).await
      }

      #[instrument(skip(self))]
      async fn fetch_model_by_index(
        &self,
        index_name: String,
        index_value: EitherSlug,
      ) -> Result<Option<Self::Model>, FetchModelByIndexError> {
        self
          .base_repo
          .fetch_model_by_index(index_name, index_value)
          .await
      }

      #[instrument(skip(self))]
      async fn enumerate_models(&self) -> Result<Vec<Self::Model>> {
        self.base_repo.enumerate_models().await
      }
    }
  };
}
