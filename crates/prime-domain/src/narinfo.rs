//! Narinfo types and impl.

use miette::{Context, IntoDiagnostic};
use models::{
  Entry, Signature, StorePath,
  dvf::{EitherSlug, EntityName, LaxSlug},
  nix_compat::narinfo::{Flags, NarInfo},
};

use crate::PrimeDomainService;

/// The request struct for the [`narinfo`](PrimeDomainService::narinfo) fn.
#[derive(Debug)]
pub struct NarinfoRequest {
  /// The name of the cache the entry is stored in.
  pub cache_name: EntityName,
  /// The store path of the entry.
  pub store_path: StorePath<String>,
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
  /// The requested cache was not found.
  #[error("The requested cache was not found: \"{0}\"")]
  CacheNotFound(EntityName),
  /// The requested entry was not found.
  #[error("The requested entry was not found: \"{0}\"")]
  EntryNotFound(StorePath<String>),
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
        "name".into(),
        EitherSlug::Strict(req.cache_name.clone().into_inner()),
      )
      .await
      .into_diagnostic()
      .context("failed to search for cache")
      .map_err(NarinfoError::InternalError)?
      .ok_or(NarinfoError::CacheNotFound(req.cache_name))?;

    let index_value = EitherSlug::Lax(LaxSlug::new(format!(
      "{cache_id}-{entry_path}",
      cache_id = cache.id,
      entry_path = &req.store_path
    )));
    let entry = self
      .entry_repo
      .fetch_model_by_unique_index(
        "cache-id-and-entry-path".to_owned(),
        index_value,
      )
      .await
      .context("failed to search for entry")
      .map_err(NarinfoError::InternalError)?
      .ok_or(NarinfoError::EntryNotFound(req.store_path.clone()))?;

    let url = format!("/download/{store_path}", store_path = entry.store_path);

    Ok(NarinfoResponse {
      entry,
      nar_relative_url: url,
    })
  }
}
