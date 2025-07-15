//! Narinfo types and impl.

use models::{StorePath, nix_compat::narinfo::NarInfo};

/// The request struct for the [`narinfo`](PrimeDomainService::narinfo) fn.
#[derive(Debug)]
pub struct NarinfoRequest {
  /// The store path of the entry.
  pub store_path: StorePath<String>,
}

/// The response struct for the [`narinfo`](PrimeDomainService::narinfo) fn.
#[derive(Debug)]
pub struct NarinfoResponse {
  /// The requested narinfo.
  pub narinfo: NarInfo<'static>,
}

/// The error enum for the [`narinfo`](PrimeDomainService::narinfo) fn.
#[derive(thiserror::Error, Debug)]
pub enum NarinfoError {
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}
