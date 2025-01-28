//! Provides a repository for the [`Store`] domain model.

use hex::health::{self, HealthAware};
pub use models::{Store, StoreCreateRequest};
use tracing::instrument;

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, Database};

/// Descriptor trait for repositories that handle [`Store`] domain model.
#[async_trait::async_trait]
pub trait StoreRepository:
  ModelRepositoryLike<
  Model = Store,
  ModelCreateRequest = StoreCreateRequest,
  CreateError = CreateModelError,
>
{
}

impl<T> StoreRepository for T where
  T: ModelRepositoryLike<
    Model = Store,
    ModelCreateRequest = StoreCreateRequest,
    CreateError = CreateModelError,
  >
{
}

/// The repository for the [`Store`] domain model.
#[derive(Clone)]
pub struct StoreRepositoryCanonical {
  base_repo: BaseRepository<Store>,
}

impl StoreRepositoryCanonical {
  /// Create a new instance of the [`Store`] repository.
  pub fn new(db: Database<Store>) -> Self {
    tracing::info!("creating new `StoreRepositoryCanonical` instance");
    Self {
      base_repo: BaseRepository::new(db),
    }
  }
}

crate::impl_repository_on_base!(
  StoreRepositoryCanonical,
  Store,
  StoreCreateRequest,
  CreateModelError
);
