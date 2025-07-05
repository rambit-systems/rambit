mod entry_data;

use dvf::{EitherSlug, LaxSlug, RecordId};
use model::{Model, SlugFieldGetter};
use nix_compat::store_path::StorePath;
use serde::{Deserialize, Serialize};

pub use self::entry_data::*;
use crate::cache::Cache;

/// An entry.
///
/// Entries have a store-and-path unique index to prevent storage collisions,
/// and a cache-and-path unique index to allow querying and prevent entry
/// duplication within a cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  /// The entry's ID.
  id:                RecordId<Entry>,
  /// The [`Cache`]s that this entry is accessible from.
  caches:            Vec<RecordId<Cache>>,
  /// The store path that the entry refers to.
  store_path:        StorePath<String>,
  /// Intrensic data about the entry's NAR.
  intrensic_data:    NarIntrensicData,
  /// Data about how the NAR exists in the [`Store`].
  storage_data:      NarStorageData,
  /// Authenticity data about the entry.
  authenticity_data: NarAuthenticityData,
  /// Data about the NAR's deriver.
  deriver_data:      NarDeriverData,
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
  /// `cache-id-and-entry-path`.
  pub fn unique_index_cache_id_and_entry_path(&self) -> Vec<EitherSlug> {
    self
      .caches
      .iter()
      .map(|cache_id| {
        EitherSlug::Lax(LaxSlug::new(format!(
          "{cache_id}-{entry_path}",
          entry_path = self.store_path
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
      "cache-id-and-entry-path",
      Entry::unique_index_cache_id_and_entry_path,
    ),
  ];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
