mod errors;

use hex::Hexagonal;
use kv::*;
use miette::Result;

pub use self::errors::*;

/// An adapter for a model-based database.
#[async_trait::async_trait]
pub(crate) trait DatabaseAdapter<M: model::Model>: Hexagonal {
  /// Creates a new model.
  async fn create_model(&self, model: M) -> Result<M, CreateModelError>;
  /// Fetches a model by its ID.
  async fn fetch_model_by_id(
    &self,
    id: model::RecordId<M>,
  ) -> Result<Option<M>, FetchModelError>;
  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's
  /// [`UNIQUE_INDICES`](model::Model::UNIQUE_INDICES) constant.
  async fn fetch_model_by_unique_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<M>, FetchModelByIndexError>;
  /// Fetches the models that match the index value.
  ///
  /// Must be a valid index, defined in the model's
  /// [`INDICES`](model::Model::INDICES) constant.
  async fn fetch_models_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Vec<M>, FetchModelByIndexError>;
  /// Produces a list of all model IDs.
  async fn enumerate_models(&self) -> Result<Vec<M>>;
}
