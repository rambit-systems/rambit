//! Download types.

mod execute;
mod plan;
#[cfg(test)]
mod tests;

use models::{
  StorePath, User,
  dvf::{EntityName, RecordId},
};

pub use self::{execute::*, plan::*};

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
