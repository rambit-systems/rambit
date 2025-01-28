//! Provides a repository for the [`Token`] domain model.

use hex::health::{self, HealthAware};
pub use models::{Token, TokenCreateRequest};
use tracing::instrument;

use super::*;
pub use crate::base::CreateModelError;
use crate::base::{BaseRepository, Database};

/// Descriptor trait for repositories that handle [`Token`] domain model.
#[async_trait::async_trait]
pub trait TokenRepository:
  ModelRepositoryLike<
  Model = Token,
  ModelCreateRequest = TokenCreateRequest,
  CreateError = CreateModelError,
>
{
}

impl<T> TokenRepository for T where
  T: ModelRepositoryLike<
    Model = Token,
    ModelCreateRequest = TokenCreateRequest,
    CreateError = CreateModelError,
  >
{
}

/// The repository for the [`Token`] domain model.
#[derive(Clone)]
pub struct TokenRepositoryCanonical {
  base_repo: BaseRepository<Token>,
}

impl TokenRepositoryCanonical {
  /// Create a new instance of the [`Token`] repository.
  pub fn new(db: Database<Token>) -> Self {
    tracing::info!("creating new `TokenRepositoryCanonical` instance");
    Self {
      base_repo: BaseRepository::new(db),
    }
  }
}

crate::impl_repository_on_base!(
  TokenRepositoryCanonical,
  Token,
  TokenCreateRequest,
  CreateModelError
);
