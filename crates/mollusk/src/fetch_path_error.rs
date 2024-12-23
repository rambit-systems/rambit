use http::StatusCode;
use miette::Diagnostic;
use serde::{Deserialize, Serialize};

use crate::{
  common::{
    NonExistentCacheError, UnauthenticatedCacheAccessError,
    UnauthorizedCacheAccessError,
  },
  InternalError, MalformedTokenSecretError, MissingPathError, MolluskError,
  NonExistentTokenError,
};

/// An error that occurs when preparing to fetch a payload.
#[derive(thiserror::Error, Diagnostic, Debug, Serialize, Deserialize)]
pub enum FetchPathError {
  /// No matching cache was found.
  #[error(transparent)]
  NoMatchingCache(#[from] NonExistentCacheError),
  /// The cache access was unauthenticated (no token supplied).
  #[error(transparent)]
  UnauthenticatedCacheAccess(#[from] UnauthenticatedCacheAccessError),
  /// The cache access was unauthorized (token supplied but insufficient).
  #[error(transparent)]
  UnauthorizedCacheAccess(#[from] UnauthorizedCacheAccessError),
  /// The supplied token does not exist.
  #[error(transparent)]
  NonExistentToken(#[from] NonExistentTokenError),
  /// The token secret was malformed.
  #[error(transparent)]
  MalformedTokenSecret(#[from] MalformedTokenSecretError),
  /// The path is missing.
  #[error(transparent)]
  MissingPath(#[from] MissingPathError),
  /// Internal error
  #[error(transparent)]
  InternalError(#[from] InternalError),
}

crate::delegate_mollusk_error!(
  FetchPathError,
  NoMatchingCache,
  UnauthenticatedCacheAccess,
  UnauthorizedCacheAccess,
  NonExistentToken,
  MalformedTokenSecret,
  MissingPath,
  InternalError,
);
