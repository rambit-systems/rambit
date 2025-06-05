use dvf::{
  RecordId,
  slugger::{EitherSlug, LaxSlug},
};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::{cache::Cache, store::Store};

/// An entry.
///
/// Entries have a store-and-path unique index to prevent storage collisions,
/// and a cache-and-path unique index to allow querying and prevent entry
/// duplication within a cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  /// The entry's ID.
  pub id:     RecordId<Entry>,
  /// The entry's store.
  pub store:  RecordId<Store>,
  /// The entry's nix path.
  pub path:   LaxSlug,
  /// The [`Cache`]s that this entry is accessible from.
  pub caches: Vec<RecordId<Cache>>,
}

impl Entry {
  /// Generates the value of the unique [`Entry`] index
  /// `store-id-and-entry-path`.
  pub fn unique_index_store_id_and_entry_path(&self) -> Vec<EitherSlug> {
    vec![EitherSlug::Lax(LaxSlug::new(format!(
      "{store_id}-{entry_path}",
      store_id = self.id,
      entry_path = self.path
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
          entry_path = self.path
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
