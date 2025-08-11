use std::fmt;

use dvf::{EitherSlug, RecordId};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::User;

/// An org.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Org {
  /// The org's ID.
  pub id:        RecordId<Org>,
  /// The org's identifier.
  pub org_ident: OrgIdent,
}

impl Org {
  /// Generates the value of the unique [`Org`] index `ident`.
  pub fn unique_index_ident(&self) -> Vec<EitherSlug> {
    match self.org_ident {
      OrgIdent::Named(ref entity_name) => {
        vec![entity_name.clone().into_inner().into()]
      }
      OrgIdent::UserOrg(_) => Vec::new(),
    }
  }
}

/// The unique index selector for [`Org`].
#[derive(Debug, Clone, Copy)]
pub enum OrgUniqueIndexSelector {
  /// The `ident` index.
  Ident,
}

impl fmt::Display for OrgUniqueIndexSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      OrgUniqueIndexSelector::Ident => write!(f, "ident"),
    }
  }
}

/// The [`Org`]'s identifier.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrgIdent {
  /// An [`Org`] identifier using a name.
  Named(dvf::EntityName),
  /// An [`Org`] identifier using a user ID.
  UserOrg(RecordId<User>),
}

impl Model for Org {
  type IndexSelector = !;
  type UniqueIndexSelector = OrgUniqueIndexSelector;

  const INDICES: &'static [(Self::IndexSelector, SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = "org";
  const UNIQUE_INDICES: &'static [(
    Self::UniqueIndexSelector,
    SlugFieldGetter<Self>,
  )] = &[(OrgUniqueIndexSelector::Ident, Org::unique_index_ident)];

  fn id(&self) -> RecordId<Org> { self.id }
}
