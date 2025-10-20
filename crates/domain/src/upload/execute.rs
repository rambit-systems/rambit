use std::path::PathBuf;

use belt::Belt;
use miette::{Context, IntoDiagnostic};
use models::{
  Entry, NarAuthenticityData, NarStorageData,
  dvf::{CompressionStatus, RecordId},
  model::Model,
};
use serde::{Deserialize, Serialize};

use super::plan::UploadPlan;
use crate::DomainService;

/// The response struct for the
/// [`execute_upload`](DomainService::execute_upload) fn.
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
  /// The ID of the created entry.
  pub entry_id: RecordId<Entry>,
}

/// The error enum for the [`execute_upload`](DomainService::execute_upload)
/// fn.
#[derive(thiserror::Error, Debug)]
pub enum UploadExecutionError {
  /// Failed to write to storage.
  #[error("Failed to write to storage: {0}")]
  StorageFailure(storage::WriteError),
  /// Failed to read all the input data.
  #[error("Failed to read input data: {0}")]
  InputDataError(std::io::Error),
  /// Failed to validate NAR.
  #[error("Failed to validate NAR: {0}")]
  NarValidationError(owl::InterrogatorError),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Uploads a payload to storage, creates an entry, and adds it to a cache.
  #[tracing::instrument(skip(self))]
  pub async fn execute_upload(
    &self,
    plan: UploadPlan,
  ) -> Result<UploadResponse, UploadExecutionError> {
    let entry_id = RecordId::new();

    // WARNING: buffer all the data right now because we need it to validate
    // the NAR and to upload to storage
    let big_terrible_buffer = plan
      .nar_contents
      .collect()
      .await
      .map_err(UploadExecutionError::InputDataError)?;

    // validate the NAR and gather intrensic data
    let nar_interrogator = owl::NarInterrogator;
    let mut nar_intrensic_data = nar_interrogator
      .interrogate(Belt::from_bytes(big_terrible_buffer.clone(), None))
      .await
      .map_err(UploadExecutionError::NarValidationError)?;

    // remove any self-reference from the intrensic data
    let removed_self_reference =
      nar_intrensic_data.references.remove(&plan.store_path);
    if !removed_self_reference {
      tracing::warn!("no self-reference found in entry {entry_id}");
    }

    let store_client = storage::StorageClient::new_from_storage_creds(
      plan.target_store.credentials.into(),
    )
    .await
    .map_err(UploadExecutionError::InternalError)?;

    let storage_path = PathBuf::from(plan.store_path.to_string());
    let file_size = store_client
      .write(
        storage_path.as_ref(),
        Belt::from_bytes(big_terrible_buffer, None),
      )
      .await
      .map_err(UploadExecutionError::StorageFailure)?;
    let compression_status =
      CompressionStatus::Uncompressed { size: file_size };

    let nar_storage_data = NarStorageData {
      store: plan.target_store.id,
      storage_path,
      compression_status,
    };

    let nar_authenticity_data = NarAuthenticityData::default();

    // insert entry
    let entry_id = self
      .mutate
      .create_entry(Entry {
        id:                entry_id,
        org:               plan.org_id,
        caches:            plan.caches.iter().map(Model::id).collect(),
        store_path:        plan.store_path,
        intrensic_data:    nar_intrensic_data,
        storage_data:      nar_storage_data,
        authenticity_data: nar_authenticity_data,
        deriver_data:      plan.deriver_data,
      })
      .await
      .into_diagnostic()
      .context("failed to create entry")
      .map_err(UploadExecutionError::InternalError)?;

    Ok(UploadResponse { entry_id })
  }
}
