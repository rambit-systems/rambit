use serde::{Deserialize, Serialize};

use crate::{Model, RecordId};

/// An org.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Org {
  /// The org's ID.
  pub id:   RecordId<Org>,
  /// The org's name.
  pub name: dvf::EntityName,
}

impl Model for Org {
  const TABLE_NAME: &'static str = "org";

  const UNIQUE_INDICES: &'static [(
    &'static str,
    crate::SlugFieldGetter<Self>,
  )] = &[("name", |org| org.name.clone().into_inner().into())];
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];

  fn id(&self) -> RecordId<Org> { self.id }
}
