mod nar_data;

use std::{fmt, str::FromStr};

use dvf::{EitherSlug, LaxSlug, RecordId};
use model::{Model, SlugFieldGetter};
pub use nix_compat::{
  narinfo::Signature, nixhash::CAHash, store_path::StorePath,
};
use nix_compat::{nixbase32, store_path::DIGEST_SIZE};
use serde::{Deserialize, Serialize};

pub use self::nar_data::*;
use crate::cache::Cache;

/// A store path digest.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Digest([u8; DIGEST_SIZE]);

impl Digest {
  /// Provides access to the inner buffer.
  pub fn inner(&self) -> &[u8; DIGEST_SIZE] { &self.0 }
}

impl fmt::Display for Digest {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", nixbase32::encode(&self.0))
  }
}

impl FromStr for Digest {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(nixbase32::decode_fixed(s.as_bytes()).map_err(|_| ())?))
  }
}

/// An entry.
///
/// Entries have a store-and-path unique index to prevent storage collisions,
/// and a cache-and-path unique index to allow querying and prevent entry
/// duplication within a cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  /// The entry's ID.
  pub id:                RecordId<Entry>,
  /// The [`Cache`]s that this entry is accessible from.
  pub caches:            Vec<RecordId<Cache>>,
  /// The store path that the entry refers to.
  pub store_path:        StorePath<String>,
  /// Intrensic data about the entry's NAR.
  pub intrensic_data:    NarIntrensicData,
  /// Data about how the NAR exists in the [`Store`](super::Store).
  pub storage_data:      NarStorageData,
  /// Authenticity data about the entry.
  pub authenticity_data: NarAuthenticityData,
  /// Data about the NAR's deriver.
  pub deriver_data:      NarDeriverData,
}

impl Entry {
  /// Generates the value of the unique [`Entry`] index
  /// `store-id-and-entry-path`.
  pub fn unique_index_store_id_and_entry_path(&self) -> Vec<EitherSlug> {
    vec![EitherSlug::Lax(LaxSlug::new(format!(
      "{store_id}-{entry_path}",
      store_id = self.id,
      entry_path = self.store_path
    )))]
  }

  /// Generates the value of the unique [`Entry`] index
  /// `cache-id-and-entry-digest`.
  pub fn unique_index_cache_id_and_entry_digest(&self) -> Vec<EitherSlug> {
    self
      .caches
      .iter()
      .map(|cache_id| {
        EitherSlug::Lax(LaxSlug::new(format!(
          "{cache_id}-{entry_digest:x?}",
          entry_digest = self.store_path.digest()
        )))
      })
      .collect()
  }
}

impl Model for Entry {
  const INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = "entry";
  const UNIQUE_INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] = &[
    (
      "store-id-and-entry-path",
      Entry::unique_index_store_id_and_entry_path,
    ),
    (
      "cache-id-and-entry-digest",
      Entry::unique_index_cache_id_and_entry_digest,
    ),
  ];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
