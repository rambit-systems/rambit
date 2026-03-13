//! Entry deletion types.

use miette::Context;
use models::{Entry, RecordId};
use storage::BlobKey;

use crate::DomainService;

/// The error enum for the [`delete_entry`](DomainService::delete_entry) fn.
#[derive(thiserror::Error, Debug)]
pub enum EntryDeletionError {
  /// Indicates that the entry does not exist.
  #[error("Failed to find entry: {0}")]
  MissingEntry(RecordId<Entry>),
  /// Failed to write to storage.
  #[error("Failed to write to storage: {0}")]
  StorageFailure(#[from] storage::BlobStorageError),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Deletes an [`Entry`].
  #[tracing::instrument(skip(self))]
  pub async fn delete_entry(
    &self,
    entry_id: RecordId<Entry>,
  ) -> Result<Entry, EntryDeletionError> {
    let entry = self
      .meta()
      .fetch_entry_by_id(entry_id)
      .await
      .map_err(|e| EntryDeletionError::InternalError(e.into()))?
      .ok_or(EntryDeletionError::MissingEntry(entry_id))?;
    let store_id = entry.storage_data.store;
    let store = self
      .meta()
      .fetch_store_by_id(store_id)
      .await
      .map_err(|e| EntryDeletionError::InternalError(e.into()))?
      .ok_or(EntryDeletionError::InternalError(miette::miette!(
        "missing store ({store_id}) referenced by entry ({entry_id})"
      )))?;

    let storage_client =
      crate::storage_glue::storage_creds_to_blob_storage(store.credentials)
        .await
        .context("failed to create storage client for store")
        .map_err(EntryDeletionError::InternalError)?;
    let storage_key =
      BlobKey::new(entry.storage_data.storage_path.to_string_lossy());

    storage_client
      .delete(&storage_key)
      .await
      .context("failed to delete entry blob from store")
      .map_err(EntryDeletionError::InternalError)?;

    let entry = self
      .mutate
      .delete_entry(entry_id)
      .await
      .map_err(|e| EntryDeletionError::InternalError(e.into()))?;

    Ok(entry)
  }
}
