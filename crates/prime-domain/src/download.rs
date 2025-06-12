//! Download types.

use std::path::PathBuf;

use belt::Belt;
use miette::{Context, IntoDiagnostic, miette};
use models::{
  User,
  dvf::{EitherSlug, EntityName, FileSize, LaxSlug, RecordId, Visibility},
};

use crate::PrimeDomainService;

/// The request struct for the [`download`](PrimeDomainService::download) fn.
#[derive(Debug)]
pub struct DownloadRequest {
  /// The downloading user's authentication.
  pub auth:         Option<RecordId<User>>,
  /// The name of the cache to look for the path in.
  pub cache_name:   EntityName,
  /// The entry's path.
  pub desired_path: LaxSlug,
}

/// The response struct for the [`download`](PrimeDomainService::download) fn.
#[derive(Debug)]
pub struct DownloadResponse {
  /// The data being downloaded.
  pub data:      Belt,
  /// The file size of the data being downloaded.
  pub file_size: FileSize,
}

/// The error enum for the [`download`](PrimeDomainService::download) fn.
#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
  /// The user is unauthorized to download from this cache.
  #[error("The user is unauthorized to download from this cache")]
  Unauthorized,
  /// The requested cache was not found.
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
  /// The requested entry was not found.
  #[error(
    "The requested entry was not found: path \"{path}\" in cache \"{cache}\""
  )]
  EntryNotFound {
    /// The cache.
    cache: EntityName,
    /// The entry path.
    path:  LaxSlug,
  },
  /// Failed to read from storage.
  #[error("Failed to read from storage: {0}")]
  StorageFailure(storage::ReadError),
}

impl PrimeDomainService {
  /// Downloads an entry's payload from storage.
  pub async fn download(
    &self,
    req: DownloadRequest,
  ) -> Result<DownloadResponse, DownloadError> {
    let cache = self
      .cache_repo
      .fetch_model_by_unique_index(
        "name".into(),
        EitherSlug::Strict(req.cache_name.clone().into_inner()),
      )
      .await
      .into_diagnostic()
      .context("failed to search for cache")
      .map_err(DownloadError::InternalError)?
      .ok_or(DownloadError::CacheNotFound(req.cache_name.clone()))?;

    let user = match req.auth {
      Some(auth) => Some(
        self
          .user_repo
          .fetch_model_by_id(auth)
          .await
          .into_diagnostic()
          .context("failed to find user")
          .map_err(DownloadError::InternalError)?
          .ok_or(miette!("authenticated user not found"))
          .map_err(DownloadError::InternalError)?,
      ),
      None => None,
    };

    match (cache.visibility, user) {
      (Visibility::Private, None) => {
        return Err(DownloadError::Unauthorized);
      }
      (Visibility::Private, Some(user)) => {
        if user.org != cache.org {
          return Err(DownloadError::Unauthorized);
        }
      }
      (Visibility::Public, _) => (),
    }

    let entry = self
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
      .context("failed to search for entry")
      .map_err(DownloadError::InternalError)?
      .ok_or(DownloadError::EntryNotFound {
        cache: cache.name,
        path:  req.desired_path.clone(),
      })?;

    let store = self
      .store_repo
      .fetch_model_by_id(entry.store)
      .await
      .into_diagnostic()
      .context("failed to find store")
      .map_err(DownloadError::InternalError)?
      .ok_or(miette!("store not found"))
      .map_err(DownloadError::InternalError)?;

    let store_client =
      storage::StorageClient::new_from_storage_creds(store.credentials)
        .await
        .map_err(DownloadError::InternalError)?;

    let path = PathBuf::from(req.desired_path.clone().into_inner());
    let data = store_client
      .read(&path)
      .await
      .map_err(DownloadError::StorageFailure)?;
    let file_size = entry.meta.file_size;

    Ok(DownloadResponse { data, file_size })
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use belt::Belt;
  use models::dvf::{EntityName, LaxSlug, RecordId, StrictSlug};

  use crate::{
    PrimeDomainService, download::DownloadRequest, upload::UploadRequest,
  };

  #[tokio::test]
  async fn test_download() {
    let pds = PrimeDomainService::mock_prime_domain().await;

    let user_id = RecordId::from_str("01JXGXV4R6VCZWQ2DAYDWR1VXD").unwrap();
    let data = bytes::Bytes::from("hello world");
    let data_belt = Belt::from_bytes(data.clone(), None);
    let cache_name = EntityName::new(StrictSlug::confident("aaron"));
    let desired_path =
      LaxSlug::confident("8r4xxbrvb9fmv9j0m224q7cb4jr5y1pa-file-5.46");
    let target_store = None;

    let upload_req = UploadRequest {
      data: data_belt,
      auth: user_id,
      cache_name: cache_name.clone(),
      desired_path: desired_path.clone(),
      target_store,
    };

    let upload_resp = pds.upload(upload_req).await.expect("failed to upload");

    let entry = pds
      .entry_repo
      .fetch_model_by_id(upload_resp.entry_id)
      .await
      .expect("failed to find entry")
      .expect("failed to find entry");

    assert_eq!(entry.path, desired_path.clone());
    // upload complete

    let download_req = DownloadRequest {
      auth: None,
      cache_name,
      desired_path,
    };

    let download_resp = pds
      .download(download_req)
      .await
      .expect("failed to download");

    let downloaded_data = download_resp
      .data
      .collect()
      .await
      .expect("failed to collect bytes from belt");
    assert_eq!(data, downloaded_data);
  }
}
