use std::{path::Path, sync::Arc};

use hex::{health, Hexagonal};
use storage::{belt::Belt, ReadError, StorageClient, WriteError};

/// The definition for the user storage service. This produces a type-erased
/// client that does the real work, since we need different clients for
/// different storage backends.
#[async_trait::async_trait]
pub(crate) trait UserStorageRepositoryLike: Hexagonal {
  /// Connects to user storage and returns a client.
  async fn connect_to_user_storage(
    &self,
    creds: models::StorageCredentials,
  ) -> miette::Result<UserStorageClient>;
}

/// The user storage service.
#[derive(Clone)]
pub struct UserStorageRepository {
  inner: Arc<dyn UserStorageRepositoryLike>,
}

#[async_trait::async_trait]
impl health::HealthReporter for UserStorageRepository {
  fn name(&self) -> &'static str { stringify!(UserStorageRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}

impl UserStorageRepository {
  /// Create a new instance of the user storage service.
  #[allow(
    clippy::new_without_default,
    reason = "Service construction still should not be flippant, despite this \
              being stateless."
  )]
  pub fn new() -> Self {
    Self {
      inner: Arc::new(UserStorageRepositoryStorageImpl::new()),
    }
  }

  /// Connects to user storage and returns a client.
  pub async fn connect_to_user_storage(
    &self,
    creds: models::StorageCredentials,
  ) -> miette::Result<UserStorageClient> {
    self.inner.connect_to_user_storage(creds).await
  }
}

/// The definition for the user storage client, produced by the
/// [`UserStorageRepository`].
#[async_trait::async_trait]
pub(crate) trait UserStorageClientLike: Hexagonal {
  /// Reads a file. Returns a [`Belt`].
  async fn read(&self, path: &Path) -> Result<Belt, ReadError>;
  /// Writes a file. Consumes a [`Belt`].
  async fn write(
    &self,
    path: &Path,
    data: Belt,
  ) -> Result<models::FileSize, WriteError>;
}

/// The user storage client.
#[derive(Clone)]
pub struct UserStorageClient {
  inner: Arc<dyn UserStorageClientLike>,
}

impl UserStorageClient {
  /// Reads a file. Returns a [`Belt`].
  pub async fn read(&self, path: &Path) -> Result<Belt, ReadError> {
    self.inner.read(path).await
  }

  /// Writes a file. Consumes a [`Belt`].
  pub async fn write(
    &self,
    path: &Path,
    data: Belt,
  ) -> Result<models::FileSize, WriteError> {
    self.inner.write(path, data).await
  }
}

/// The canonical implementation user storage client.
pub(crate) struct UserStorageClientStorageImpl(StorageClient);

impl UserStorageClientStorageImpl {
  fn new(client: StorageClient) -> Self { Self(client) }
}

#[async_trait::async_trait]
impl health::HealthReporter for UserStorageClientStorageImpl {
  fn name(&self) -> &'static str { self.0.name() }
  async fn health_check(&self) -> health::ComponentHealth {
    self.0.health_check().await
  }
}

#[async_trait::async_trait]
impl UserStorageClientLike for UserStorageClientStorageImpl {
  async fn read(&self, path: &Path) -> Result<Belt, ReadError> {
    self.0.read(path).await
  }
  async fn write(
    &self,
    path: &Path,
    data: Belt,
  ) -> Result<models::FileSize, WriteError> {
    self.0.write(path, data).await
  }
}

pub(crate) struct UserStorageRepositoryStorageImpl {}

impl UserStorageRepositoryStorageImpl {
  /// Create a new instance of the canonical user storage service.
  #[allow(
    clippy::new_without_default,
    reason = "Service construction still should not be flippant, despite this \
              being stateless."
  )]
  pub fn new() -> Self {
    tracing::info!("creating new `UserStorageServiceCanonical` instance");
    Self {}
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for UserStorageRepositoryStorageImpl {
  fn name(&self) -> &'static str { stringify!(UserStorageServiceCanonical) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::IntrensicallyUp.into()
  }
}

#[async_trait::async_trait]
impl UserStorageRepositoryLike for UserStorageRepositoryStorageImpl {
  async fn connect_to_user_storage(
    &self,
    creds: models::StorageCredentials,
  ) -> miette::Result<UserStorageClient> {
    Ok(UserStorageClient {
      inner: Arc::new(UserStorageClientStorageImpl::new(
        StorageClient::new_from_storage_creds(creds).await?,
      )),
    })
  }
}
