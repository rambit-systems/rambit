use dvf::{HumanName, RecordId};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::Org;

/// A user.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  /// The user's ID.
  pub id:   RecordId<User>,
  /// The user's org.
  pub org:  RecordId<Org>,
  /// The user's name.
  pub name: HumanName,
}

impl Model for User {
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = "user";
  const UNIQUE_INDICES: &'static [(&'static str, SlugFieldGetter<Self>)] = &[];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
