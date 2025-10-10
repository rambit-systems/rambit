//! Download types.

use belt::Belt;
use miette::{Context, IntoDiagnostic, miette};
use models::{
  Digest, StorePath, User,
  dvf::{
    CompressionAlgorithm, CompressionStatus, EntityName, FileSize, RecordId,
    Visibility,
  },
};

use crate::DomainService;

/// The request struct for the [`download`](DomainService::download) fn.
#[derive(Debug)]
pub struct DownloadRequest {
  /// The downloading user's authentication.
  pub auth:       Option<RecordId<User>>,
  /// The name of the cache to look for the path in.
  pub cache_name: EntityName,
  /// The entry's store path.
  pub store_path: StorePath<String>,
}

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
pub enum DownloadError {
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
  pub async fn download(
    &self,
    req: DownloadRequest,
  ) -> Result<DownloadResponse, DownloadError> {
    let cache = self
      .meta
      .fetch_cache_by_name(req.cache_name.clone())
      .await
      .into_diagnostic()
      .context("failed to search for cache")
      .map_err(DownloadError::InternalError)?
      .ok_or(DownloadError::CacheNotFound(req.cache_name.clone()))?;

    let user = match req.auth {
      Some(auth) => Some(
        self
          .meta
          .fetch_user_by_id(auth)
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
        if !user.belongs_to_org(cache.org) {
          return Err(DownloadError::Unauthorized);
        }
      }
      (Visibility::Public, _) => (),
    }

    let entry = self
      .meta
      .fetch_entry_by_cache_id_and_entry_digest(
        cache.id,
        Digest::from_bytes(*req.store_path.digest()),
      )
      .await
      .into_diagnostic()
      .context("failed to search for entry")
      .map_err(DownloadError::InternalError)?
      .ok_or(DownloadError::EntryNotFound {
        cache:      cache.name,
        store_path: req.store_path.clone(),
      })?;

    let store = self
      .meta
      .fetch_store_by_id(entry.storage_data.store)
      .await
      .into_diagnostic()
      .context("failed to find store")
      .map_err(DownloadError::InternalError)?
      .ok_or(miette!("store not found"))
      .map_err(DownloadError::InternalError)?;

    let store_client =
      storage::StorageClient::new_from_storage_creds(store.credentials.into())
        .await
        .map_err(DownloadError::InternalError)?;

    let path = entry.storage_data.storage_path;
    let comp_status = entry.storage_data.compression_status;
    let data = store_client
      .read(&path)
      .await
      .map_err(DownloadError::StorageFailure)?;
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

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use belt::Belt;
  use bytes::Bytes;
  use models::{
    NarDeriverData, StorePath,
    dvf::{EntityName, RecordId, StrictSlug},
  };

  use crate::{
    DomainService, download::DownloadRequest, upload::UploadRequest,
  };

  #[tokio::test]
  async fn test_download() {
    let pds = DomainService::mock_domain().await;

    let input_bytes = Bytes::from_static(include_bytes!(
      "../../owl/test/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0"
    ));
    let nar_contents = Belt::from_bytes(input_bytes.clone(), None);

    let user_id = RecordId::from_str("01JXGXV4R6VCZWQ2DAYDWR1VXD").unwrap();
    let cache_name = EntityName::new(StrictSlug::confident("aaron"));
    let target_store = EntityName::new(StrictSlug::confident("albert"));
    let store_path = "/nix/store/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0";
    let store_path =
      StorePath::from_absolute_path(store_path.as_bytes()).unwrap();

    let deriver_path =
      "/nix/store/4yz8qa58nmysad5w88rgdhq15rkssqr6-bat-0.25.0.drv".to_string();
    let deriver_path = StorePath::from_absolute_path(
      deriver_path.strip_suffix(".drv").unwrap().as_bytes(),
    )
    .unwrap();
    let deriver_data = NarDeriverData {
      system:  Some("aarch64-linux".to_string()),
      deriver: Some(deriver_path),
    };

    let req = UploadRequest {
      nar_contents,
      auth: user_id,
      caches: vec![cache_name.clone()],
      target_store,
      store_path: store_path.clone(),
      deriver_data,
    };

    let resp = pds.upload(req).await.expect("failed to upload");

    let _entry = pds
      .entry_repo
      .fetch_model_by_id(resp.entry_id)
      .await
      .expect("failed to find entry")
      .expect("failed to find entry");
    // upload complete

    let download_req = DownloadRequest {
      auth: None,
      cache_name,
      store_path,
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
    assert_eq!(input_bytes, downloaded_data);
  }
}
