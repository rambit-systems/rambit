use std::fmt;

use dvf::{EitherSlug, EntityName, LaxSlug, RecordId, StorageCredentials};
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

/// The public view of [`Store`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PvStore {
  /// The store's ID.
  pub id:     RecordId<Store>,
  /// The store's org.
  pub org:    RecordId<Org>,
  /// The store's configuration.
  pub config: StoreConfiguration,
  /// The store's nickname.
  pub name:   EntityName,
}

impl From<Store> for PvStore {
  fn from(value: Store) -> Self {
    PvStore {
      id:     value.id,
      org:    value.org,
      config: value.config,
      name:   value.name,
    }
  }
}

/// The unique index selector for [`Store`].
#[derive(Debug, Clone, Copy)]
pub enum StoreUniqueIndexSelector {
  /// The `name` index.
  Name,
}

impl fmt::Display for StoreUniqueIndexSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StoreUniqueIndexSelector::Name => write!(f, "name"),
    }
  }
}

/// The index selector for [`Store`].
#[derive(Debug, Clone, Copy)]
pub enum StoreIndexSelector {
  /// The `org` index.
  Org,
}

impl fmt::Display for StoreIndexSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StoreIndexSelector::Org => write!(f, "org"),
    }
  }
}

/// The configuration for a [`Store`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoreConfiguration {}

impl Model for Store {
  type IndexSelector = StoreIndexSelector;
  type UniqueIndexSelector = StoreUniqueIndexSelector;

  const INDICES: &'static [(
    Self::IndexSelector,
    model::SlugFieldGetter<Self>,
  )] = &[(StoreIndexSelector::Org, |s| {
    vec![LaxSlug::new(s.org.to_string()).into()]
  })];
  const TABLE_NAME: &'static str = "store";
  const UNIQUE_INDICES: &'static [(
    Self::UniqueIndexSelector,
    SlugFieldGetter<Self>,
  )] = &[(StoreUniqueIndexSelector::Name, Store::unique_index_name)];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
