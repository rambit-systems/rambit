//! Narinfo types and impl.

use miette::{Context, IntoDiagnostic, miette};
use models::{
  CacheUniqueIndexSelector, Digest, Entry, EntryUniqueIndexSelector, Signature,
  StorePath, User,
  dvf::{EitherSlug, EntityName, RecordId, Visibility},
  nix_compat::narinfo::{Flags, NarInfo},
};

use crate::PrimeDomainService;

/// The request struct for the [`narinfo`](PrimeDomainService::narinfo) fn.
#[derive(Debug)]
pub struct NarinfoRequest {
  /// The user's authentication.
  pub auth:       Option<RecordId<User>>,
  /// The name of the cache the entry is stored in.
  pub cache_name: EntityName,
  /// The store path digest of the entry.
  pub digest:     Digest,
}

/// The response struct for the [`narinfo`](PrimeDomainService::narinfo) fn.
#[derive(Debug)]
pub struct NarinfoResponse {
  entry:            Entry,
  nar_relative_url: String,
}

impl NarinfoResponse {
  /// Returns the requested [`NarInfo`].
  pub fn narinfo(&self) -> NarInfo<'_> {
    let mut references = self
      .entry
      .intrensic_data
      .references
      .iter()
      .map(StorePath::as_ref)
      .collect::<Vec<_>>();
    references.sort_unstable();
    let mut signatures = self
      .entry
      .authenticity_data
      .signatures
      .iter()
      .map(Signature::as_ref)
      .collect::<Vec<_>>();
    signatures.sort_unstable_by_key(|s| s.to_string());

    NarInfo {
      flags: Flags::empty(),
      store_path: self.entry.store_path.as_ref(),
      nar_hash: self.entry.intrensic_data.nar_hash,
      nar_size: self.entry.intrensic_data.nar_size.into_inner(),
      references,
      signatures,
      ca: self.entry.intrensic_data.ca_hash.clone(),
      system: self.entry.deriver_data.system.as_deref(),
      deriver: self
        .entry
        .deriver_data
        .deriver
        .as_ref()
        .map(StorePath::as_ref),
      url: &self.nar_relative_url,
      compression: None,
      file_hash: Some(self.entry.intrensic_data.nar_hash),
      file_size: Some(self.entry.intrensic_data.nar_size.into_inner()),
    }
  }
}

/// The error enum for the [`narinfo`](PrimeDomainService::narinfo) fn.
#[derive(thiserror::Error, Debug)]
pub enum NarinfoError {
  /// The user is unauthorized to read from this cache.
  #[error("The user is unauthorized to read from this cache")]
  Unauthorized,
  /// The requested cache was not found.
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
  /// The requested entry was not found.
  #[error("The requested entry was not found: \"{0}\"")]
  EntryNotFound(Digest),
  /// Some other internal error.
  #[error("Unexpected error: {0}")]
  InternalError(miette::Report),
}

impl PrimeDomainService {
  /// Calculates the narinfo for a given entry.
  pub async fn narinfo(
    &self,
    req: NarinfoRequest,
  ) -> Result<NarinfoResponse, NarinfoError> {
    let cache = self
      .cache_repo
      .fetch_model_by_unique_index(
        CacheUniqueIndexSelector::Name,
        EitherSlug::Strict(req.cache_name.clone().into_inner()),
      )
      .await
      .into_diagnostic()
      .context("failed to search for cache")
      .map_err(NarinfoError::InternalError)?
      .ok_or(NarinfoError::CacheNotFound(req.cache_name))?;

    let user = match req.auth {
      Some(user_id) => Some(
        self
          .user_repo
          .fetch_model_by_id(user_id)
          .await
          .into_diagnostic()
          .context("failed to find user")
          .map_err(NarinfoError::InternalError)?
          .ok_or(miette!("authenticated user not found"))
          .map_err(NarinfoError::InternalError)?,
      ),
      None => None,
    };

    // reject user if cache is private and org IDs don't match
    match (cache.visibility, user) {
      // let them through if it's private but org IDs match
      (Visibility::Private, Some(user)) if user.belongs_to_org(cache.org) => (),
      // otherwise don't let them through if it's private
      (Visibility::Private, _) => {
        return Err(NarinfoError::Unauthorized);
      }
      // if it's not private don't worry about it
      _ => (),
    }

    let entry = self
      .entry_repo
      .fetch_model_by_unique_index(
        EntryUniqueIndexSelector::CacheIdAndEntryDigest,
        Entry::unique_index_cache_id_and_entry_digest(cache.id, req.digest),
      )
      .await
      .context("failed to search for entry")
      .map_err(NarinfoError::InternalError)?
      .ok_or(NarinfoError::EntryNotFound(req.digest))?;

    // note: this is a relative path from the narinfo endpoint
    let url = format!("download/{store_path}", store_path = entry.store_path);

    Ok(NarinfoResponse {
      entry,
      nar_relative_url: url,
    })
  }
}
