use miette::{Context, IntoDiagnostic, miette};
use models::{Digest, EntityName, Entry, Store, StorePath, Visibility};

use crate::{DomainService, download::DownloadRequest};

/// A download plan produced by [`plan_download`](DomainService::plan_download)
/// fn.
#[derive(Debug)]
pub struct DownloadPlan {
  /// The entry being requested.
  pub(crate) entry: Entry,
  /// The store the entry resides in.
  pub(crate) store: Store,
}

/// The error enum for the [`plan_download`](DomainService::plan_download) fn.
#[derive(thiserror::Error, Debug)]
pub enum DownloadPlanningError {
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
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Plans a download.
  #[tracing::instrument(skip(self))]
  pub async fn plan_download(
    &self,
    req: DownloadRequest,
  ) -> Result<DownloadPlan, DownloadPlanningError> {
    // fetch the cache and make sure it exists
    let cache = self
      .meta
      .fetch_cache_by_name(req.cache_name.clone())
      .await
      .into_diagnostic()
      .context("failed to search for cache")
      .map_err(DownloadPlanningError::InternalError)?
      .ok_or(DownloadPlanningError::CacheNotFound(req.cache_name.clone()))?;

    // fetch the user if an ID was given
    let user = match req.auth {
      Some(auth) => Some(
        self
          .meta
          .fetch_user_by_id(auth)
          .await
          .into_diagnostic()
          .context("failed to find user")
          .map_err(DownloadPlanningError::InternalError)?
          .ok_or(miette!("authenticated user not found"))
          .map_err(DownloadPlanningError::InternalError)?,
      ),
      None => None,
    };

    // authorize the user if the cache requires it
    match (cache.visibility, user) {
      (Visibility::Private, None) => {
        return Err(DownloadPlanningError::Unauthorized);
      }
      (Visibility::Private, Some(user)) => {
        if !user.belongs_to_org(cache.org) {
          return Err(DownloadPlanningError::Unauthorized);
        }
      }
      (Visibility::Public, _) => (),
    }

    // fetch the entry
    let entry = self
      .meta
      .fetch_entry_by_cache_id_and_entry_digest(
        cache.id,
        Digest::from_bytes(*req.store_path.digest()),
      )
      .await
      .into_diagnostic()
      .context("failed to search for entry")
      .map_err(DownloadPlanningError::InternalError)?
      .ok_or(DownloadPlanningError::EntryNotFound {
        cache:      cache.name.clone(),
        store_path: req.store_path.clone(),
      })?;

    // fetch the store the entry resides in
    let store = self
      .meta
      .fetch_store_by_id(entry.storage_data.store)
      .await
      .into_diagnostic()
      .context("failed to find store")
      .map_err(DownloadPlanningError::InternalError)?
      .ok_or(miette!("store not found"))
      .map_err(DownloadPlanningError::InternalError)?;

    Ok(DownloadPlan { entry, store })
  }
}
