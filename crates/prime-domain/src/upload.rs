use belt::Belt;
use models::{
  Entry,
  dvf::{EntityName, LaxSlug, RecordId},
};

use crate::PrimeDomainService;

pub struct UploadRequest {
  data:         Belt,
  #[allow(dead_code)]
  auth:         (),
  cache_name:   EntityName,
  desired_path: LaxSlug,
}

pub struct UploadResponse {
  entry_id: RecordId<Entry>,
}

#[derive(thiserror::Error, Debug)]
pub enum UploadError {
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
}

impl PrimeDomainService {
  /// Uploads a payload to storage, creates an entry, and adds it to a cache.
  pub async fn upload(
    _req: UploadRequest,
  ) -> Result<UploadResponse, UploadError> {
    todo!()
  }
}
