mod creds;

use model::{IndexValue, Model, RecordId};
use model_types::EntityName;
use serde::{Deserialize, Serialize};

pub use self::creds::*;
use crate::Org;

/// A store.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(
  table = "store",
  index(name = "org", extract = |m| vec![IndexValue::new_single(m.org.to_string())]),
  index(name = "name_by_org", unique, extract =
    |m| vec![Store::unique_index_name_by_org(m.org, &m.name)]
  ),
)]
pub struct Store {
  /// The store's ID.
  #[model(id)]
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
  /// `name_by_org`.
  pub fn unique_index_name_by_org(
    org: RecordId<Org>,
    name: &EntityName,
  ) -> IndexValue {
    IndexValue::new([org.to_string(), name.to_string()])
  }
}

/// The public view of [`Store`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PvStore {
  /// The store's ID.
  pub id:          RecordId<Store>,
  /// The store's org.
  pub org:         RecordId<Org>,
  /// The store's credentials.
  pub credentials: PvStorageCredentials,
  /// The store's configuration.
  pub config:      StoreConfiguration,
  /// The store's nickname.
  pub name:        EntityName,
}

impl From<Store> for PvStore {
  fn from(value: Store) -> Self {
    PvStore {
      id:          value.id,
      org:         value.org,
      credentials: value.credentials.into(),
      config:      value.config,
      name:        value.name,
    }
  }
}

/// The configuration for a [`Store`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreConfiguration {}
