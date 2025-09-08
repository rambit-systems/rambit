mod creds;

use std::fmt;

use dvf::{EitherSlug, EntityName, LaxSlug, RecordId};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

pub use self::creds::*;
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
  /// `name_by_org`.
  pub fn unique_index_name_by_org(&self) -> EitherSlug {
    LaxSlug::new(format!(
      "{org_id}-{name}",
      org_id = self.org,
      name = self.name
    ))
    .into()
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

/// The unique index selector for [`Store`].
#[derive(Debug, Clone, Copy)]
pub enum StoreUniqueIndexSelector {
  /// The `name-by-org` index.
  NameByOrg,
}

impl fmt::Display for StoreUniqueIndexSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StoreUniqueIndexSelector::NameByOrg => write!(f, "name-by-org"),
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
  )] = &[(StoreUniqueIndexSelector::NameByOrg, |s| {
    vec![Store::unique_index_name_by_org(s)]
  })];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
