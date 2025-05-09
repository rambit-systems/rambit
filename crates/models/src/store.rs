use serde::{Deserialize, Serialize};

use crate::{HumanName, Model, Org, RecordId, StorageCredentials};

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
}

/// The configuration for a [`Store`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreConfiguration {}

impl Model for Store {
  const TABLE_NAME: &'static str = "store";

  const UNIQUE_INDICES: &'static [(
    &'static str,
    crate::SlugFieldGetter<Self>,
  )] = &[];
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
