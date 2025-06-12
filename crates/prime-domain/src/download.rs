//! Download types.

use models::{
  User,
  dvf::{EntityName, LaxSlug, RecordId},
};

use crate::PrimeDomainService;

/// The request struct for the [`download`](PrimeDomainService::download) fn.
#[derive(Debug)]
pub struct DownloadRequest {
  /// The downloading user's authentication.
  pub auth:         RecordId<User>,
  /// The name of the cache to look for the path in.
  pub cache_name:   EntityName,
  /// The entry's path.
  pub desired_path: LaxSlug,
}

/// The response struct for the [`download`](PrimeDomainService::download) fn.
#[derive(Debug)]
pub struct DownloadResponse {}

/// The error enum for the [`download`](PrimeDomainService::download) fn.
#[derive(thiserror::Error, Debug)]
pub enum DownloadError {}

impl PrimeDomainService {
  /// Downloads an entry's payload from storage.
  pub async fn download(
    _req: DownloadRequest,
  ) -> Result<DownloadResponse, DownloadError> {
    todo!()
  }
}
