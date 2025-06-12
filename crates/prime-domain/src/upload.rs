//! Upload types.

use std::path::PathBuf;

use belt::Belt;
use miette::{Context, IntoDiagnostic, miette};
use models::{
  Entry, EntryMetadata, User,
  dvf::{EitherSlug, EntityName, LaxSlug, RecordId},
};
use serde::{Deserialize, Serialize};

use crate::PrimeDomainService;

/// The request struct for the [`upload`](PrimeDomainService::upload) fn.
#[derive(Debug)]
pub struct UploadRequest {
  /// The data to be uploaded.
  pub data:         Belt,
  /// The uploading user's authentication.
  pub auth:         RecordId<User>,
  /// The name of the cache to register the entry in.
  pub cache_name:   EntityName,
  /// The entry's path.
  pub desired_path: LaxSlug,
  /// The store to store the data in.
  pub target_store: Option<EntityName>,
}

/// The response struct for the [`upload`](PrimeDomainService::upload) fn.
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
  /// The ID of the created entry.
  pub entry_id: RecordId<Entry>,
}

/// The error enum for the [`upload`](PrimeDomainService::upload) fn.
#[derive(thiserror::Error, Debug)]
pub enum UploadError {
  /// The user is unauthorized to upload to this cache.
  #[error("The user is unauthorized to upload to this cache")]
  Unauthorized,
  /// The requested cache was not found.
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
  /// The target store was not found.
  #[error("The target store was not found: \"{0}\"")]
  TargetStoreNotFound(EntityName),
  /// An entry with that path already exists in the target store.
  #[error("An entry with that path already exists in the target store: {0}")]
  DuplicateEntryInStore(RecordId<Entry>),
  /// An entry with that path already exists in the cache.
  #[error("An entry with that path already exists in the cache: {0}")]
  DuplicateEntryInCache(RecordId<Entry>),
  /// Failed to write to storage.
  #[error("Failed to write to storage: {0}")]
  StorageFailure(storage::WriteError),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl PrimeDomainService {
  /// Uploads a payload to storage, creates an entry, and adds it to a cache.
  pub async fn upload(
    &self,
    req: UploadRequest,
  ) -> Result<UploadResponse, UploadError> {
    let cache = self
      .cache_repo
      .fetch_model_by_unique_index(
        "name".into(),
        EitherSlug::Strict(req.cache_name.clone().into_inner()),
      )
      .await
      .into_diagnostic()
      .context("failed to search for cache")
      .map_err(UploadError::InternalError)?
      .ok_or(UploadError::CacheNotFound(req.cache_name.clone()))?;

    let user = self
      .user_repo
      .fetch_model_by_id(req.auth)
      .await
      .into_diagnostic()
      .context("failed to find user")
      .map_err(UploadError::InternalError)?
      .ok_or(miette!("authenticated user not found"))
      .map_err(UploadError::InternalError)?;

    if user.org != cache.org {
      return Err(UploadError::Unauthorized);
    }

    let target_store = match req.target_store {
      Some(store_name) => self
        .store_repo
        .fetch_model_by_unique_index(
          "name".into(),
          EitherSlug::Strict(store_name.clone().into_inner()),
        )
        .await
        .into_diagnostic()
        .context("failed to search for target store")
        .map_err(UploadError::InternalError)?
        .ok_or(UploadError::TargetStoreNotFound(store_name))?,
      None => self
        .store_repo
        .fetch_model_by_id(cache.default_store)
        .await
        .into_diagnostic()
        .context("failed to find store")
        .map_err(UploadError::InternalError)?
        .ok_or(miette!("store not found"))
        .map_err(UploadError::InternalError)?,
    };

    // make sure no entry exists for this path and store
    let duplicate_entry_by_store = self
      .entry_repo
      .fetch_model_by_unique_index(
        "store-id-and-entry-path".into(),
        EitherSlug::Lax(LaxSlug::new(format!(
          "{store_id}-{entry_path}",
          store_id = target_store.id,
          entry_path = req.desired_path
        ))),
      )
      .await
      .into_diagnostic()
      .context("failed to search for conflicting entries by store and path")
      .map_err(UploadError::InternalError)?;

    if let Some(entry) = duplicate_entry_by_store {
      return Err(UploadError::DuplicateEntryInStore(entry.id));
    }

    // make sure no entry exists for this path and cache
    let duplicate_entry_by_cache = self
      .entry_repo
      .fetch_model_by_unique_index(
        "cache-id-and-entry-path".into(),
        EitherSlug::Lax(LaxSlug::new(format!(
          "{cache_id}-{entry_path}",
          cache_id = cache.id,
          entry_path = req.desired_path
        ))),
      )
      .await
      .into_diagnostic()
      .context("failed to search for conflicting entries by cache and path")
      .map_err(UploadError::InternalError)?;

    if let Some(entry) = duplicate_entry_by_cache {
      return Err(UploadError::DuplicateEntryInCache(entry.id));
    }

    let store_client =
      storage::StorageClient::new_from_storage_creds(target_store.credentials)
        .await
        .map_err(UploadError::InternalError)?;

    let path = PathBuf::from(req.desired_path.clone().into_inner());
    let file_size = store_client
      .write(path.as_ref(), req.data)
      .await
      .map_err(UploadError::StorageFailure)?;

    // insert entry
    let entry = self
      .entry_repo
      .create_model(Entry {
        id:     RecordId::new(),
        store:  target_store.id,
        path:   req.desired_path,
        caches: vec![cache.id],
        meta:   EntryMetadata { file_size },
      })
      .await
      .into_diagnostic()
      .context("failed to create entry")
      .map_err(UploadError::InternalError)?;

    Ok(UploadResponse { entry_id: entry.id })
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use belt::Belt;
  use db::kv::{LaxSlug, StrictSlug};
  use models::dvf::{EntityName, RecordId};

  use super::UploadRequest;
  use crate::PrimeDomainService;

  #[tokio::test]
  async fn test_upload() {
    let pds = PrimeDomainService::mock_prime_domain().await;

    let user_id = RecordId::from_str("01JXGXV4R6VCZWQ2DAYDWR1VXD").unwrap();
    let data = Belt::from_bytes(bytes::Bytes::from("hello world"), None);
    let cache_name = EntityName::new(StrictSlug::confident("aaron"));
    let desired_path =
      LaxSlug::confident("8r4xxbrvb9fmv9j0m224q7cb4jr5y1pa-file-5.46");
    let target_store = None;

    let req = UploadRequest {
      data,
      auth: user_id,
      cache_name,
      desired_path: desired_path.clone(),
      target_store,
    };

    let resp = pds.upload(req).await.expect("failed to upload");

    let entry = pds
      .entry_repo
      .fetch_model_by_id(resp.entry_id)
      .await
      .expect("failed to find entry")
      .expect("failed to find entry");

    assert_eq!(entry.path, desired_path);
  }
}
