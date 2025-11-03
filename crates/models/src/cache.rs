use model::{IndexValue, Model, RecordId};
use model_types::{EntityName, Visibility};
use serde::{Deserialize, Serialize};

use crate::Org;

/// A cache.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(
  table = "cache",
  index(name = "name", unique, extract =
    |m| vec![IndexValue::new_single(&m.name)]
  ),
  index(name = "org", extract =
    |m| vec![IndexValue::new_single(m.org.to_string())]
  ),
)]
pub struct Cache {
  /// The cache's ID.
  #[model(id)]
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
