use belt::Belt;
use models::{
  StorePath,
  dvf::{CompressionAlgorithm, CompressionStatus, EntityName, FileSize},
};

use super::plan::DownloadPlan;
use crate::DomainService;

/// The response struct for the [`download`](DomainService::download) fn.
#[derive(Debug)]
pub struct DownloadResponse {
  /// The data being downloaded.
  pub data:      Belt,
  /// The file size of the data being downloaded.
  pub file_size: FileSize,
}

/// The error enum for the [`download`](DomainService::download) fn.
#[derive(thiserror::Error, Debug)]
pub enum DownloadExecutionError {
  /// The user is unauthorized to download from this cache.
  #[error("The user is unauthorized to download from this cache")]
  Unauthorized,
  /// The requested cache was not found.
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
  /// The requested entry was not found.
  #[error(
    "The requested entry was not found: store path \"{store_path}\" in cache \
     \"{cache}\""
  )]
  EntryNotFound {
    /// The cache.
    cache:      EntityName,
    /// The entry store path.
    store_path: StorePath<String>,
  },
  /// Failed to read from storage.
  #[error("Failed to read from storage: {0}")]
  StorageFailure(storage::ReadError),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Downloads an entry's payload from storage.
  pub async fn execute_download(
    &self,
    plan: DownloadPlan,
  ) -> Result<DownloadResponse, DownloadExecutionError> {
    let store_client = storage::StorageClient::new_from_storage_creds(
      plan.store.credentials.into(),
    )
    .await
    .map_err(DownloadExecutionError::InternalError)?;

    let path = plan.entry.storage_data.storage_path;
    let comp_status = plan.entry.storage_data.compression_status;
    let data = store_client
      .read(&path)
      .await
      .map_err(DownloadExecutionError::StorageFailure)?;
    let (file_size, data) = match comp_status {
      CompressionStatus::Compressed {
        uncompressed_size,
        algorithm,
        ..
      } => {
        let data = data
          .set_declared_comp(Some(match algorithm {
            CompressionAlgorithm::Zstd => belt::CompressionAlgorithm::Zstd,
          }))
          .adapt_to_no_comp();
        (uncompressed_size, data)
      }
      CompressionStatus::Uncompressed { size } => (size, data),
    };

    Ok(DownloadResponse { data, file_size })
  }
}
