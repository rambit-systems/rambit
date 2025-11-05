use models::{
  MemoryStorageCredentials, R2StorageCredentials, StorageCredentials,
};
use storage::{BlobStorage, BlobStorageResult};

pub async fn storage_creds_to_blob_storage(
  creds: StorageCredentials,
) -> BlobStorageResult<BlobStorage> {
  match creds {
    StorageCredentials::Memory(MemoryStorageCredentials) => {
      Ok(BlobStorage::new_memory())
    }
    StorageCredentials::R2(R2StorageCredentials::Default {
      access_key,
      secret_access_key,
      endpoint,
      bucket,
    }) => BlobStorage::new_s3_bucket(
      &bucket,
      "auto",
      &endpoint,
      Some(&access_key),
      Some(&secret_access_key),
    ),
  }
}
