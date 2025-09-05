use std::fmt;

use dvf::{EitherSlug, EntityName, LaxSlug, RecordId, Visibility};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::Org;

/// A cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
  /// The cache's ID.
  pub id:         RecordId<Cache>,
  /// The cache's org.
  pub org:        RecordId<Org>,
  /// The cache's name.
  pub name:       EntityName,
  /// The cache's base visibility.
  pub visibility: Visibility,
}

/// The public view of [`Cache`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PvCache {
  /// The cache's ID.
  pub id:         RecordId<Cache>,
  /// The cache's org.
  pub org:        RecordId<Org>,
  /// The cache's name.
  pub name:       EntityName,
  /// The cache's base visibility.
  pub visibility: Visibility,
}

impl From<Cache> for PvCache {
  fn from(value: Cache) -> Self {
    PvCache {
      id:         value.id,
      org:        value.org,
      name:       value.name,
      visibility: value.visibility,
    }
  }
}

/// The unique index selector for [`Cache`].
#[derive(Debug, Clone, Copy)]
pub enum CacheUniqueIndexSelector {
  /// The `name` index.
  Name,
}

impl fmt::Display for CacheUniqueIndexSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CacheUniqueIndexSelector::Name => write!(f, "name"),
    }
  }
}

/// The index selector for [`Cache`].
#[derive(Debug, Clone, Copy)]
pub enum CacheIndexSelector {
  /// The `org` index.
  Org,
}

impl fmt::Display for CacheIndexSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CacheIndexSelector::Org => write!(f, "org"),
    }
  }
}

impl Cache {
  /// Generates the value of the unique [`Cache`] index
  /// `name`.
  pub fn unique_index_name(&self) -> Vec<EitherSlug> {
    vec![self.name.clone().into_inner().into()]
  }
}

impl Model for Cache {
  type IndexSelector = CacheIndexSelector;
  type UniqueIndexSelector = CacheUniqueIndexSelector;

  const INDICES: &'static [(
    Self::IndexSelector,
    model::SlugFieldGetter<Self>,
  )] = &[(CacheIndexSelector::Org, |c| {
    vec![LaxSlug::new(c.org.to_string()).into()]
  })];
  const TABLE_NAME: &'static str = "cache";
  const UNIQUE_INDICES: &'static [(
    Self::UniqueIndexSelector,
    SlugFieldGetter<Self>,
  )] = &[(CacheUniqueIndexSelector::Name, Cache::unique_index_name)];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
