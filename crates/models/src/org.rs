use dvf::RecordId;
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

/// An org.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Org {
  /// The org's ID.
  pub id:   RecordId<Org>,
  /// The org's name.
  pub name: dvf::EntityName,
}

impl Model for Org {
  const INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = "org";
  const UNIQUE_INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] =
    &[("name", |org| org.name.clone().into_inner().into())];

  fn id(&self) -> RecordId<Org> { self.id }
}
