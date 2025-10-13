//! Upload types.

use std::path::PathBuf;

use belt::Belt;
use meta_domain::SearchByUserError;
use miette::{Context, IntoDiagnostic, miette};
use models::{
  Cache, Digest, Entry, NarAuthenticityData, NarDeriverData, NarStorageData,
  Org, Store, StorePath, User,
  dvf::{CompressionStatus, EntityName, RecordId},
  model::Model,
};
use serde::{Deserialize, Serialize};

use crate::DomainService;

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

/// The upload plan produced by [`plan_upload`](DomainService::plan_upload) fn.
#[derive(Debug)]
pub struct UploadPlan {
  /// The data to be uploaded.
  nar_contents: Belt,
  /// The store path of the entry.
  store_path:   StorePath<String>,
  /// The store to store the data in.
  target_store: Store,
  /// The org that everything is scoped to.
  org_id:       RecordId<Org>,
  /// The caches for the entry to be registered in.
  caches:       Vec<Cache>,
  /// Data about the NAR's deriver
  deriver_data: NarDeriverData,
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
  /// Multiple stores were found with the given name in different organizations.
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
  /// Failed to write to storage.
  #[error("Failed to write to storage: {0}")]
  StorageFailure(storage::WriteError),
  /// Failed to read all the input data.
  #[error("Failed to read input data: {0}")]
  InputDataError(std::io::Error),
  /// Failed to validate NAR.
  #[error("Failed to validate NAR: {0}")]
  NarValidationError(owl::InterrogatorError),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

/// The response struct for the
/// [`execute_upload`](DomainService::execute_upload) fn.
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
  /// The ID of the created entry.
  pub entry_id: RecordId<Entry>,
}

/// The error enum for the [`execute_upload`](DomainService::execute_upload) fn.
#[derive(thiserror::Error, Debug)]
pub enum UploadExecutionError {
  /// The user is unauthorized to upload to this cache.
  #[error("The user is unauthorized to upload to this cache")]
  Unauthorized,
  /// The requested cache was not found.
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
  /// The target store was not found.
  #[error("The target store was not found: \"{0}\"")]
  TargetStoreNotFound(EntityName),
  /// Multiple stores were found with the given name in different organizations.
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
  /// Failed to write to storage.
  #[error("Failed to write to storage: {0}")]
  StorageFailure(storage::WriteError),
  /// Failed to read all the input data.
  #[error("Failed to read input data: {0}")]
  InputDataError(std::io::Error),
  /// Failed to validate NAR.
  #[error("Failed to validate NAR: {0}")]
  NarValidationError(owl::InterrogatorError),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Plans an upload.
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
        SearchByUserError::FetchError(e) => UploadPlanningError::InternalError(
          Err::<(), _>(e)
            .into_diagnostic()
            .context("failed to search for stores by user")
            .unwrap_err(),
        ),
        SearchByUserError::FetchByIndexError(e) => {
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

    Ok(UploadPlan {
      nar_contents: req.nar_contents,
      store_path: req.store_path,
      target_store,
      org_id,
      caches,
      deriver_data: req.deriver_data,
    })
  }

  /// Uploads a payload to storage, creates an entry, and adds it to a cache.
  pub async fn execute_upload(
    &self,
    plan: UploadPlan,
  ) -> Result<UploadResponse, UploadExecutionError> {
    let entry_id = RecordId::new();

    // WARNING: buffer all the data right now because we need it to validate the
    // NAR and to upload to storage
    let big_terrible_buffer = plan
      .nar_contents
      .collect()
      .await
      .map_err(UploadExecutionError::InputDataError)?;

    // validate the NAR and gather intrensic data
    let nar_interrogator = owl::NarInterrogator;
    let mut nar_intrensic_data = nar_interrogator
      .interrogate(Belt::from_bytes(big_terrible_buffer.clone(), None))
      .await
      .map_err(UploadExecutionError::NarValidationError)?;

    // remove any self-reference from the intrensic data
    let removed_self_reference =
      nar_intrensic_data.references.remove(&plan.store_path);
    if !removed_self_reference {
      tracing::warn!("no self-reference found in entry {entry_id}");
    }

    let store_client = storage::StorageClient::new_from_storage_creds(
      plan.target_store.credentials.into(),
    )
    .await
    .map_err(UploadExecutionError::InternalError)?;

    let storage_path = PathBuf::from(plan.store_path.to_string());
    let file_size = store_client
      .write(
        storage_path.as_ref(),
        Belt::from_bytes(big_terrible_buffer, None),
      )
      .await
      .map_err(UploadExecutionError::StorageFailure)?;
    let compression_status =
      CompressionStatus::Uncompressed { size: file_size };

    let nar_storage_data = NarStorageData {
      store: plan.target_store.id,
      storage_path,
      compression_status,
    };

    let nar_authenticity_data = NarAuthenticityData::default();

    // insert entry
    let entry = self
      .entry_repo
      .create_model(Entry {
        id:                entry_id,
        org:               plan.org_id,
        caches:            plan.caches.iter().map(Model::id).collect(),
        store_path:        plan.store_path,
        intrensic_data:    nar_intrensic_data,
        storage_data:      nar_storage_data,
        authenticity_data: nar_authenticity_data,
        deriver_data:      plan.deriver_data,
      })
      .await
      .into_diagnostic()
      .context("failed to create entry")
      .map_err(UploadExecutionError::InternalError)?;

    Ok(UploadResponse { entry_id: entry.id })
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use belt::Belt;
  use bytes::Bytes;
  use db::kv::StrictSlug;
  use models::{
    NarDeriverData, StorePath,
    dvf::{EntityName, RecordId},
  };

  use super::UploadRequest;
  use crate::DomainService;

  #[tokio::test]
  async fn test_upload() {
    let pds = DomainService::mock_domain().await;

    let bytes = Bytes::from_static(include_bytes!(
      "../../owl/test/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0"
    ));
    let nar_contents = Belt::from_bytes(bytes, None);

    let user_id = RecordId::from_str("01JXGXV4R6VCZWQ2DAYDWR1VXD").unwrap();
    let caches = vec![EntityName::new(StrictSlug::confident("aaron"))];
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
      caches,
      target_store,
      store_path,
      deriver_data,
    };

    let plan = pds.plan_upload(req).await.expect("failed to plan upload");
    let resp = pds
      .execute_upload(plan)
      .await
      .expect("failed to execute upload");

    let _entry = pds
      .entry_repo
      .fetch_model_by_id(resp.entry_id)
      .await
      .expect("failed to find entry")
      .expect("failed to find entry");
  }
}
