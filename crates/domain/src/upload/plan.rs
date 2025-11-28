use belt::Belt;
use meta_domain::SearchByUserError;
use metrics::compute::UnstampedComputeUsageEvent;
use miette::{Context, IntoDiagnostic, miette};
use models::{
  Cache, Digest, EntityName, Entry, NarDeriverData, Org, RecordId, Store,
  StorePath,
};

use super::UploadRequest;
use crate::DomainService;

/// The upload plan produced by [`plan_upload`](DomainService::plan_upload)
/// fn.
#[derive(Debug)]
pub struct UploadPlan {
  /// The data to be uploaded.
  pub(crate) nar_contents:  Belt,
  /// The store path of the entry.
  pub(crate) store_path:    StorePath<String>,
  /// The store to store the data in.
  pub(crate) target_store:  Store,
  /// The org that everything is scoped to.
  pub(crate) org_id:        RecordId<Org>,
  /// The caches for the entry to be registered in.
  pub(crate) caches:        Vec<Cache>,
  /// Data about the NAR's deriver
  pub(crate) deriver_data:  NarDeriverData,
  /// The compute event to be sent.
  pub(crate) compute_event: UnstampedComputeUsageEvent,
}

/// The error enum produced by [`plan_upload`](DomainService::plan_upload) fn.
#[derive(thiserror::Error, Debug)]
pub enum UploadPlanningError {
  /// The user is unauthorized to upload to this cache.
  #[error("The user is unauthorized to upload to this cache")]
  Unauthorized,
  /// The requested cache was not found.
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
  /// The target store was not found.
  #[error("The target store was not found: \"{0}\"")]
  TargetStoreNotFound(EntityName),
  /// Multiple stores were found with the given name in different
  /// organizations.
  #[error(
    "The target store name \"{1}\" is ambiguous: multiple results found in \
     orgs {0:?}"
  )]
  TargetStoreAmbiguous(Vec<RecordId<Org>>, EntityName),
  /// An entry with that path already exists in the target store.
  #[error("An entry with that path already exists in the target store: {0}")]
  DuplicateEntryInStore(RecordId<Entry>),
  /// An entry with that path already exists in the cache.
  #[error(
    "An entry with that path already exists in the cache: entry {entry} in \
     cache {cache}"
  )]
  DuplicateEntryInCache {
    /// The entry that already exists in the cache.
    entry: RecordId<Entry>,
    /// The cache that contains the duplicate.
    cache: RecordId<Cache>,
  },
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Plans an upload.
  #[tracing::instrument(skip(self))]
  pub async fn plan_upload(
    &self,
    req: UploadRequest,
  ) -> Result<UploadPlan, UploadPlanningError> {
    // find the user
    let user = self
      .meta
      .fetch_user_by_id(req.auth)
      .await
      .into_diagnostic()
      .context("failed to find user")
      .map_err(UploadPlanningError::InternalError)?
      .ok_or(miette!("authenticated user not found"))
      .map_err(UploadPlanningError::InternalError)?;

    // find the stores the user could be referring to
    let possible_stores = self
      .meta
      .search_stores_by_name_and_user(user.id, req.target_store.clone())
      .await
      .map_err(|e| match e {
        SearchByUserError::MissingUser(u) => {
          unreachable!("user {u} was already fetched")
        }
        SearchByUserError::DatabaseError(e) => {
          UploadPlanningError::InternalError(
            Err::<(), _>(e)
              .into_diagnostic()
              .context("failed to search for stores by user")
              .unwrap_err(),
          )
        }
      })?;

    // make sure there's only one
    let target_store = match possible_stores.len() {
      0 => Err(UploadPlanningError::TargetStoreNotFound(req.target_store)),
      1 => Ok(possible_stores.first().unwrap().clone()),
      _ => Err(UploadPlanningError::TargetStoreAmbiguous(
        possible_stores.iter().map(|s| s.org).collect(),
        req.target_store,
      )),
    }?;

    // org is assigned by the store
    let org_id = target_store.org;

    // make sure the user owns the store
    if !user.belongs_to_org(org_id) {
      return Err(UploadPlanningError::Unauthorized);
    }

    // find all the caches specified
    let mut caches = Vec::with_capacity(req.caches.len());
    for cache_name in req.caches {
      caches.push(
        self
          .meta
          .fetch_cache_by_name(cache_name.clone())
          .await
          .into_diagnostic()
          .context("failed to search for cache")
          .map_err(UploadPlanningError::InternalError)?
          .ok_or(UploadPlanningError::CacheNotFound(cache_name))?,
      );
    }

    // reject request if any cache lies outside the org
    if caches.iter().any(|c| org_id != c.org) {
      return Err(UploadPlanningError::Unauthorized);
    }

    // make sure no entry exists for this path and store
    let duplicate_entry_by_store = self
      .meta
      .fetch_entry_by_store_id_and_entry_path(target_store.id, &req.store_path)
      .await
      .into_diagnostic()
      .context("failed to search for conflicting entries by store and path")
      .map_err(UploadPlanningError::InternalError)?;

    if let Some(entry) = duplicate_entry_by_store {
      return Err(UploadPlanningError::DuplicateEntryInStore(entry.id));
    }

    // make sure no entry exists for this path and any targeted cache
    for cache in caches.iter() {
      let duplicate_entry_by_cache = self
        .meta
        .fetch_entry_by_cache_id_and_entry_digest(
          cache.id,
          Digest::from_bytes(*req.store_path.digest()),
        )
        .await
        .into_diagnostic()
        .context("failed to search for conflicting entries by cache and path")
        .map_err(UploadPlanningError::InternalError)?;

      if let Some(entry) = duplicate_entry_by_cache {
        return Err(UploadPlanningError::DuplicateEntryInCache {
          entry: entry.id,
          cache: cache.id,
        });
      }
    }

    let compute_event = UnstampedComputeUsageEvent {
      entry_path: req.store_path.clone().to_absolute_path(),
      org_id,
      op_type: metrics::compute::OperationType::Upload,
    };

    Ok(UploadPlan {
      nar_contents: req.nar_contents,
      store_path: req.store_path,
      target_store,
      org_id,
      caches,
      deriver_data: req.deriver_data,
      compute_event,
    })
  }
}
