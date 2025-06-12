use dvf::{EitherSlug, EntityName, RecordId, Visibility};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::{Org, Store};

/// A cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
  /// The cache's ID.
  pub id:            RecordId<Cache>,
  /// The cache's org.
  pub org:           RecordId<Org>,
  /// The cache's name.
  pub name:          EntityName,
  /// The cache's default [`Store`].
  pub default_store: RecordId<Store>,
  /// The cache's base visibility.
  pub visibility:    Visibility,
}

impl Cache {
  /// Generates the value of the unique [`Cache`] index
  /// `name`.
  pub fn unique_index_name(&self) -> Vec<EitherSlug> {
    vec![self.name.clone().into_inner().into()]
  }
}

impl Model for Cache {
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = "cache";
  const UNIQUE_INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] =
    &[("name", Cache::unique_index_name)];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
