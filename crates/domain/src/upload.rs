//! Upload types.

mod execute;
mod plan;
#[cfg(test)]
mod tests;

use belt::Belt;
use models::{
  NarDeriverData, StorePath, User,
  dvf::{EntityName, RecordId},
};

/// The request struct for the [`upload`](DomainService::upload) fn.
#[derive(Debug)]
pub struct UploadRequest {
  /// The data to be uploaded.
  pub nar_contents: Belt,
  /// The uploading user's authentication.
  pub auth:         RecordId<User>,
  /// The name of the cache to register the entry in.
  pub caches:       Vec<EntityName>,
  /// The store to store the data in.
  pub target_store: EntityName,
  /// The store path of the entry.
  pub store_path:   StorePath<String>,
  /// Data about the NAR's deriver.
  pub deriver_data: NarDeriverData,
}
