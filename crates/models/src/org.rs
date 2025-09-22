use std::fmt;

use dvf::{EitherSlug, RecordId, StrictSlug};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::{AuthUser, User};

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
        vec![StrictSlug::new(format!("named-{}", entity_name)).into()]
      }
      OrgIdent::UserOrg(u) => {
        vec![StrictSlug::new(format!("user-{}", u)).into()]
      }
    }
  }
}

/// The public view of [`Org`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PvOrg {
  /// The org's ID.
  pub id:        RecordId<Org>,
  /// The org's identifier.
  pub org_ident: OrgIdent,
}

impl PvOrg {
  /// Returns the org's title from the perspective of a user. `None` if user
  /// shouldn't have access.
  pub fn user_facing_title(&self, user: &AuthUser) -> Option<String> {
    match &self.org_ident {
      OrgIdent::Named(entity_name) => Some(entity_name.to_string()),
      OrgIdent::UserOrg(user_id) if *user_id == user.id => {
        Some("Personal Org".to_owned())
      }
      _ => None,
    }
  }
}

impl From<Org> for PvOrg {
  fn from(value: Org) -> Self {
    PvOrg {
      id:        value.id,
      org_ident: value.org_ident,
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
