mod abbreviate;
mod digest;
mod nar_data;

use model::{IndexValue, Model, RecordId};
pub use nix_compat::{
  narinfo::Signature, nixhash::CAHash, store_path::StorePath,
};
use serde::{Deserialize, Serialize};

use self::digest::Digest;
pub use self::{abbreviate::*, nar_data::*};
use crate::{Org, Store, cache::Cache};

/// An entry.
///
/// Entries have a store-and-path unique index to prevent storage collisions,
/// and a cache-and-path unique index to allow querying and prevent entry
/// duplication within a cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(
  table = "entry",
  index(name = "org", extract = |m| vec![IndexValue::new_single(m.org.to_string())]),
  index(name = "store", extract = |m| vec![IndexValue::new_single(m.storage_data.store.to_string())]),
  index(name = "caches", extract = |m| m.caches.iter().map(|c| IndexValue::new_single(c.to_string())).collect()),
  index(name = "store_id_and_entry_path", unique, extract =
    |m| vec![Entry::unique_index_store_id_and_entry_path(m.storage_data.store, &m.store_path)]
  ),
  index(name = "cache_id_and_entry_digest", unique, extract =
    Entry::unique_index_cache_id_and_entry_digest_all
  ),
)]
pub struct Entry {
  /// The entry's ID.
  #[model(id)]
  pub id:                RecordId<Entry>,
  /// The entry's org.
  pub org:               RecordId<Org>,
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
  pub fn unique_index_store_id_and_entry_path(
    store_id: RecordId<Store>,
    entry_path: &StorePath<String>,
  ) -> IndexValue {
    IndexValue::new([store_id.to_string(), entry_path.to_string()])
  }

  /// Generates a single value of the unique [`Entry`] index
  /// `cache-id-and-entry-digest`.
  pub fn unique_index_cache_id_and_entry_digest_single(
    cache_id: RecordId<Cache>,
    entry_digest: Digest,
  ) -> IndexValue {
    IndexValue::new([cache_id.to_string(), entry_digest.to_string()])
  }

  /// Generates all values of the unique [`Entry`] index
  /// `cache-id-and-entry-digest` for a given [`Entry`].
  pub fn unique_index_cache_id_and_entry_digest_all(&self) -> Vec<IndexValue> {
    self
      .caches
      .iter()
      .map(|c| {
        Self::unique_index_cache_id_and_entry_digest_single(
          *c,
          Digest::from_bytes(*self.store_path.digest()),
        )
      })
      .collect()
  }
}
