use belt::Belt;
use futures::TryStreamExt;
use metrics_types::egress::UnstampedEgressUsageEvent;
use miette::Context;
use models::{CompressionStatus, FileSize};
use storage::{BlobKey, BlobStorageError};

use super::plan::DownloadPlan;
use crate::DomainService;

/// The response struct for the
/// [`execute_download`](DomainService::execute_download) fn.
#[derive(Debug)]
pub struct DownloadResponse {
  /// The data being downloaded.
  pub data:         Belt,
  /// The file size of the data being downloaded.
  pub file_size:    FileSize,
  /// The egress event to be sent.
  pub egress_event: UnstampedEgressUsageEvent,
}

/// The error enum for the [`execute_download`](DomainService::execute_download)
/// fn.
#[derive(thiserror::Error, Debug)]
pub enum DownloadExecutionError {
  /// Failed to read from storage.
  #[error("Failed to read from storage: {0}")]
  StorageFailure(storage::BlobStorageError),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Downloads an entry's payload from storage.
  #[tracing::instrument(skip(self, plan), fields(plan.entry.id, plan.entry.store_path))]
  pub async fn execute_download(
    &self,
    plan: DownloadPlan,
  ) -> Result<DownloadResponse, DownloadExecutionError> {
    // build a client to fetch from the store
    let store_client = crate::storage_glue::storage_creds_to_blob_storage(
      plan.store.credentials,
    )
    .await
    .context("failed to create storage client for store")
    .map_err(DownloadExecutionError::InternalError)?;

    // fetch the data from the store
    let path =
      BlobKey::new(plan.entry.storage_data.storage_path.to_string_lossy());
    let data = store_client
      .get_stream(&path)
      .await
      .map_err(DownloadExecutionError::StorageFailure)?;
    let data = Belt::new(data.map_err(BlobStorageError::into_io_error));

    // decompress if needed
    let comp_status = plan.entry.storage_data.compression_status;
    let (file_size, data) = match comp_status {
      // CompressionStatus::Compressed {
      //   uncompressed_size,
      //   algorithm,
      //   ..
      // } => {
      //   todo!()
      // }
      CompressionStatus::Uncompressed { size } => (size, data),
    };

    Ok(DownloadResponse {
      data,
      file_size,
      egress_event: plan.egress_event,
    })
  }
}
