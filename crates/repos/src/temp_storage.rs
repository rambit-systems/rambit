mod mock;

use std::sync::Arc;

use hex::{
  health::{self, HealthAware},
  Hexagonal,
};
use models::TempStoragePath;
use storage::{belt::Belt, temp::TempStorageCreds, StorageClient};
pub use storage::{
  ReadError as StorageReadError, WriteError as StorageWriteError,
};

use self::mock::TempStorageRepositoryMock;

/// Descriptor trait for repositories that handle temp storage.
#[async_trait::async_trait]
pub(crate) trait TempStorageRepositoryLike: Hexagonal {
  /// Read data from the storage.
  async fn read(&self, path: TempStoragePath)
    -> Result<Belt, StorageReadError>;
  /// Store data in the storage.
  async fn store(
    &self,
    data: Belt,
  ) -> Result<TempStoragePath, StorageWriteError>;
}

/// The repository for temp storage.
#[derive(Clone)]
pub(crate) struct TempStorageRepositoryStorageImpl {
  client: StorageClient,
}

impl TempStorageRepositoryStorageImpl {
  /// Create a new instance of the temp storage repository.
  pub async fn new(creds: TempStorageCreds) -> miette::Result<Self> {
    tracing::info!("creating new `TempStorageRepositoryCanonical` instance");
    Ok(Self {
      client: StorageClient::new_from_storage_creds(creds.as_creds()).await?,
    })
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for TempStorageRepositoryStorageImpl {
  fn name(&self) -> &'static str {
    stringify!(TempStorageRepositoryStorageImpl)
  }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.client.health_report(),
    ))
    .await
    .into()
  }
}

#[async_trait::async_trait]
impl TempStorageRepositoryLike for TempStorageRepositoryStorageImpl {
  #[tracing::instrument(skip(self))]
  async fn read(
    &self,
    path: TempStoragePath,
  ) -> Result<Belt, StorageReadError> {
    self.client.read(path.path()).await
  }

  #[tracing::instrument(skip(self, data))]
  async fn store(
    &self,
    data: Belt,
  ) -> Result<TempStoragePath, StorageWriteError> {
    let mut path = TempStoragePath::new_random(models::FileSize::new(0));
    let counter = data.counter();
    self.client.write(path.path(), data).await?;
    path.set_size(models::FileSize::new(counter.current()));
    Ok(path)
  }
}

#[derive(Clone)]
/// The repository for temp storage.
pub struct TempStorageRepository {
  inner: Arc<dyn TempStorageRepositoryLike>,
}

impl TempStorageRepository {
  /// Create a new instance of the temp storage repository from
  /// [`TempStorageCreds`].
  pub async fn new_from_creds(inner: TempStorageCreds) -> miette::Result<Self> {
    Ok(Self {
      inner: Arc::new(TempStorageRepositoryStorageImpl::new(inner).await?),
    })
  }
  /// Create a new instance of the temp storage repository from a mock.
  pub fn new_from_mock(fs_root: std::path::PathBuf) -> Self {
    Self {
      inner: Arc::new(TempStorageRepositoryMock::new(fs_root)),
    }
  }

  /// Read data from the storage.
  pub async fn read(
    &self,
    path: TempStoragePath,
  ) -> Result<Belt, StorageReadError> {
    self.inner.read(path).await
  }
  /// Store data in the storage.
  pub async fn store(
    &self,
    data: Belt,
  ) -> Result<TempStoragePath, StorageWriteError> {
    self.inner.store(data).await
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for TempStorageRepository {
  fn name(&self) -> &'static str { stringify!(TempStorageRepository) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(Some(
      self.inner.health_report(),
    ))
    .await
    .into()
  }
}
