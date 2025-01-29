use storage::belt;

use super::*;

/// A mock repository for temp storage.
#[derive(Clone)]
pub(crate) struct TempStorageRepositoryMock {
  fs_root: std::path::PathBuf,
}

impl TempStorageRepositoryMock {
  /// Create a new instance of the temp storage repository.
  pub fn new(fs_root: std::path::PathBuf) -> Self {
    tracing::info!("creating new `TempStorageRepositoryMock` instance");
    Self { fs_root }
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for TempStorageRepositoryMock {
  fn name(&self) -> &'static str { stringify!(TempStorageRepositoryMock) }

  async fn health_check(&self) -> health::ComponentHealth {
    health::IntrensicallyUp.into()
  }
}

#[async_trait::async_trait]
impl TempStorageRepositoryLike for TempStorageRepositoryMock {
  #[tracing::instrument(skip(self))]
  async fn read(
    &self,
    path: TempStoragePath,
  ) -> Result<Belt, StorageReadError> {
    let path = self.fs_root.join(path.path());
    let file = tokio::fs::File::open(path).await?;
    Ok(Belt::from_async_read(file, Some(belt::DEFAULT_CHUNK_SIZE)))
  }

  #[tracing::instrument(skip(self, data))]
  async fn store(
    &self,
    data: Belt,
  ) -> Result<TempStoragePath, StorageWriteError> {
    // create fs_root if it doesn't exist
    tokio::fs::create_dir_all(&self.fs_root).await?;

    let mut path = TempStoragePath::new_random(models::FileSize::new(0));
    let real_path = self.fs_root.join(path.path());

    let counter = data.counter();
    let mut file = tokio::fs::File::create(real_path).await?;
    tokio::io::copy(&mut data.to_async_buf_read(), &mut file).await?;
    path.set_size(models::FileSize::new(counter.current()));
    Ok(path)
  }
}
