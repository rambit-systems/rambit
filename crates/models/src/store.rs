use dvf::{EitherSlug, EntityName, RecordId, StorageCredentials};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::Org;

/// A store.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
  /// The store's ID.
  pub id:          RecordId<Store>,
  /// The store's org.
  pub org:         RecordId<Org>,
  /// The store's credentials.
  pub credentials: StorageCredentials,
  /// The store's configuration.
  pub config:      StoreConfiguration,
  /// The store's nickname.
  pub name:        EntityName,
}

impl Store {
  /// Generates the value of the unique [`Store`] index
  /// `name`.
  pub fn unique_index_name(&self) -> Vec<EitherSlug> {
    vec![self.name.clone().into_inner().into()]
  }
}

/// The configuration for a [`Store`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreConfiguration {}

impl Model for Store {
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = "store";
  const UNIQUE_INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] =
    &[("name", Store::unique_index_name)];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
