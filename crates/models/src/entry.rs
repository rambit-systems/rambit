use dvf::{
  slugger::{EitherSlug, LaxSlug},
  RecordId,
};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::store::Store;

/// An entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  /// The entry's ID.
  pub id:    RecordId<Entry>,
  /// The entry's store.
  pub store: RecordId<Store>,
  /// The entry's nix path.
  pub path:  LaxSlug,
}

impl Entry {
  /// Generates the value of the unique [`Entry`] index
  /// `store-id-and-entry-path`.
  pub fn unique_index_store_id_and_entry_path(&self) -> EitherSlug {
    LaxSlug::new(format!(
      "{store_id}-{entry_path}",
      store_id = self.id,
      entry_path = self.path
    ))
    .into()
  }
}

impl Model for Entry {
  const TABLE_NAME: &'static str = "entry";

  const UNIQUE_INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] = &[(
    "store-id-and-entry-path",
    Entry::unique_index_store_id_and_entry_path,
  )];
  const INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] = &[];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
