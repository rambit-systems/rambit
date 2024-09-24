//! Provides repository traits and implementors.

mod base;
mod cache;
mod entry;
mod store;
mod token;

use std::future::Future;

pub use db::{FetchModelByIndexError, FetchModelError};
use miette::Result;
use slugger::EitherSlug;

pub use self::{cache::*, entry::*, store::*, token::*};

/// Defines a repository interface for models.
pub trait ModelRepository: Clone + Send + Sync + 'static {
  /// The model type.
  type Model: models::Model;
  /// The request type for creating a model.
  type ModelCreateRequest: std::fmt::Debug + Send + Sync + 'static;
  /// The error type for creating a model.
  type CreateError: std::error::Error + Send + Sync + 'static;

  /// Creates a new model.
  fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> impl Future<Output = Result<(), Self::CreateError>> + Send;

  /// Fetches a model by its ID.
  fn fetch_model_by_id(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> impl Future<Output = Result<Option<Self::Model>, FetchModelError>> + Send;

  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's `INDICES` constant.
  fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> impl Future<Output = Result<Option<Self::Model>, FetchModelByIndexError>> + Send;
}

/// Defines a repository fetcher interface for models.
pub trait ModelRepositoryFetcher: Clone + Send + Sync + 'static {
  /// The model type.
  type Model: models::Model;

  /// Fetches a model by its ID.
  fn fetch(
    &self,
    id: models::RecordId<Self::Model>,
  ) -> impl Future<Output = Result<Option<Self::Model>, FetchModelError>> + Send;
}

/// Defines a repository interface for creating models.
pub trait ModelRepositoryCreator: Clone + Send + Sync + 'static {
  /// The model type.
  type Model: models::Model;
  /// The request type for creating a model.
  type ModelCreateRequest: std::fmt::Debug + Send + Sync + 'static;
  /// The error type for creating a model.
  type CreateError: std::error::Error + Send + Sync + 'static;

  /// Creates a new model.
  fn create_model(
    &self,
    input: Self::ModelCreateRequest,
  ) -> impl Future<Output = Result<(), Self::CreateError>> + Send;
}