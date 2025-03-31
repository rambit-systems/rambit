//! Provides traits and implementations for storage clients.

mod local;
mod s3_compat;
pub mod temp;

use std::{
  fmt,
  path::{Path, PathBuf},
  sync::Arc,
};

pub use belt;
use belt::Belt;
use hex::{health, Hexagonal};

use self::{local::LocalStorageClient, s3_compat::S3CompatStorageClient};

/// Trait that allows generating a dynamic client from storage credentials.
#[async_trait::async_trait]
pub(crate) trait StorageClientGenerator {
  /// Generates a dynamic client from storage credentials.
  async fn client(
    &self,
  ) -> miette::Result<Arc<dyn StorageClientLike + Send + Sync + 'static>>;
}

#[async_trait::async_trait]
impl StorageClientGenerator for dvf::StorageCredentials {
  async fn client(
    &self,
  ) -> miette::Result<Arc<dyn StorageClientLike + Send + Sync + 'static>> {
    match self {
      Self::Local(local_storage_creds) => Ok(Arc::new(
        LocalStorageClient::new(local_storage_creds.clone()).await?,
      )
        as Arc<dyn StorageClientLike + Send + Sync + 'static>),
      Self::R2(r2_storage_creds) => Ok(Arc::new(
        S3CompatStorageClient::new_r2(r2_storage_creds.clone()).await?,
      )
        as Arc<dyn StorageClientLike + Send + Sync + 'static>),
    }
  }
}

/// An error type used when reading from a `StorageClient`.
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum ReadError {
  /// The path was not found in the storage.
  #[error("the file was not available in storage: {0}")]
  NotFound(PathBuf),
  /// The path was invalid.
  #[error("the supplied path was invalid: {0}")]
  InvalidPath(String),
  /// An IO error occurred.
  #[error("a local filesystem error occurred: {0}")]
  IoError(#[from] std::io::Error),
}

/// An error type used when writing to a `StorageClient`.
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum WriteError {
  /// The path was invalid.
  #[error("the supplied path was invalid: {0}")]
  InvalidPath(String),
  /// An IO error occurred.
  #[error("a local filesystem error occurred: {0}")]
  IoError(#[from] std::io::Error),
  /// An error occurred while uploading a multipart.
  #[error("an error occurred while performing a multipart upload: {0}")]
  MultipartError(miette::Report),
}

/// The main storage trait. Allows reading to or writing from a stream of bytes.
#[async_trait::async_trait]
pub(crate) trait StorageClientLike: Hexagonal {
  /// Reads a file. Returns a [`Belt`].
  async fn read(&self, path: &Path) -> Result<Belt, ReadError>;
  /// Writes a file. Consumes a [`Belt`].
  async fn write(
    &self,
    path: &Path,
    data: Belt,
  ) -> Result<dvf::FileSize, WriteError>;
}

/// A client for interacting with a storage backend.
#[derive(Clone)]
pub struct StorageClient {
  inner: Arc<dyn StorageClientLike>,
}

impl fmt::Debug for StorageClient {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StorageClient")
      .field("inner", &stringify!(Arc<dyn StorageClientLike>))
      .finish()
  }
}

impl StorageClient {
  /// Creates a new `StorageClient` from `StorageCredentials`.
  pub async fn new_from_storage_creds(
    creds: dvf::StorageCredentials,
  ) -> miette::Result<Self> {
    let inner = creds.client().await?;
    Ok(Self { inner })
  }

  /// Reads from a path.
  pub async fn read(&self, path: &Path) -> Result<Belt, ReadError> {
    self.inner.read(path).await
  }

  /// Writes to a path.
  pub async fn write(
    &self,
    path: &Path,
    data: Belt,
  ) -> Result<dvf::FileSize, WriteError> {
    self.inner.write(path, data).await
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for StorageClient {
  fn name(&self) -> &'static str { self.inner.name() }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}
