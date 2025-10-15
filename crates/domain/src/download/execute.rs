use belt::Belt;
use models::dvf::{CompressionAlgorithm, CompressionStatus, FileSize};

use super::plan::DownloadPlan;
use crate::DomainService;

/// The response struct for the
/// [`execute_download`](DomainService::execute_download) fn.
#[derive(Debug)]
pub struct DownloadResponse {
  /// The data being downloaded.
  pub data:      Belt,
  /// The file size of the data being downloaded.
  pub file_size: FileSize,
}

/// The error enum for the [`execute_download`](DomainService::execute_download)
/// fn.
#[derive(thiserror::Error, Debug)]
pub enum DownloadExecutionError {
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
    // build a client to fetch from the store
    let store_client = storage::StorageClient::new_from_storage_creds(
      plan.store.credentials.into(),
    )
    .await
    .map_err(DownloadExecutionError::InternalError)?;

    // fetch the data from the store
    let path = plan.entry.storage_data.storage_path;
    let data = store_client
      .read(&path)
      .await
      .map_err(DownloadExecutionError::StorageFailure)?;

    // decompress if needed
    let comp_status = plan.entry.storage_data.compression_status;
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
