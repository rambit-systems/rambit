//! Provides access to the database.
//!
//! This is a hexagonal crate. It provides a [`Database`] struct.
//!
//! # [`Database`]
//! The [`Database`] struct provides a **tabular** database interface. It
//! provides CRUD operations for a generic item, using the
//! [`Model`](model::Model) trait to carry the table information.
//!
//! The implementation of the internal `DatabaseAdapter` trait is responsible
//! for organizing the database, and for bridging the gap between raw data in
//! the kv store and the model data.
//!
//! Admittedly, this is a little bit of a leaky abstraction. It
//! makes use of the [`Model`](model::Model) trait, which ideally would not be
//! involved at this level, since it's involved in the domain and the
//! [`db`](crate) crate is not.
//!
//! # Errors
//! Ideally, each method in [`Database`] should return a specific,
//! concrete error. This is the case for all but
//! [`enumerate_models`](Database::enumerate_models), which returns a
//! [`miette::Report`].

mod adapter;
mod kv_impl;
#[cfg(feature = "migrate")]
mod migrate;
mod mock_impl;

use std::{fmt, sync::Arc};

use hex::health;
pub use kv;
use kv::EitherSlug;
use miette::Result;

pub use self::adapter::*;
use self::kv_impl::KvDatabaseAdapter;
#[cfg(feature = "migrate")]
pub use self::migrate::Migrator;

/// A database.
#[derive(Clone)]
pub struct Database<M: model::Model> {
  inner: Arc<dyn DatabaseAdapter<M>>,
}

impl<M: model::Model + fmt::Debug> fmt::Debug for Database<M> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(Database))
      .field("inner", &stringify!(Arc<dyn DatabaseAdapter<M>>))
      .finish()
  }
}

impl<M: model::Model> Database<M> {
  /// Creates a new database based on a key-value store.
  pub fn new_from_kv(kv_store: kv::KeyValueStore) -> Self {
    Self {
      inner: Arc::new(KvDatabaseAdapter::new(kv_store)),
    }
  }

  /// Creates a new database based on a mocked store.
  pub fn new_mock() -> Self {
    Self {
      inner: Arc::new(self::mock_impl::MockDatabaseAdapter::default()),
    }
  }

  /// Creates a new model.
  pub async fn create_model(&self, model: M) -> Result<M, CreateModelError> {
    self.inner.create_model(model).await
  }
  /// Fetches a model by its ID.
  pub async fn fetch_model_by_id(
    &self,
    id: model::RecordId<M>,
  ) -> Result<Option<M>, FetchModelError> {
    self.inner.fetch_model_by_id(id).await
  }
  /// Fetches a model by an index.
  ///
  /// Must be a valid index, defined in the model's
  /// [`UNIQUE_INDICES`](model::Model::UNIQUE_INDICES) constant.
  pub async fn fetch_model_by_unique_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<M>, FetchModelByIndexError> {
    self
      .inner
      .fetch_model_by_unique_index(index_name, index_value)
      .await
  }
  /// Produces a list of all model IDs.
  pub async fn enumerate_models(&self) -> Result<Vec<M>> {
    self.inner.enumerate_models().await
  }
}

#[async_trait::async_trait]
impl<M: model::Model> health::HealthReporter for Database<M> {
  fn name(&self) -> &'static str { stringify!(Database<M>) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}
