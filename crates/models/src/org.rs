use model::{IndexValue, Model, RecordId};
use model_types::EntityName;
use serde::{Deserialize, Serialize};

use crate::{AuthUser, User};

/// An org.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(
  table = "org",
  index(name = "ident", unique, extract = Org::unique_index_ident)
)]
pub struct Org {
  /// The org's ID.
  #[model(id)]
  pub id:        RecordId<Org>,
  /// The org's identifier.
  pub org_ident: OrgIdent,
}

impl Org {
  /// Generates the value of the unique [`Org`] index `ident`.
  pub fn unique_index_ident(&self) -> Vec<IndexValue> {
    vec![self.org_ident.index_value()]
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

/// The [`Org`]'s identifier.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrgIdent {
  /// An [`Org`] identifier using a name.
  Named(EntityName),
  /// An [`Org`] identifier using a user ID.
  UserOrg(RecordId<User>),
}

impl OrgIdent {
  /// Calculates the unique index value for this org ident.
  pub fn index_value(&self) -> IndexValue {
    match self {
      OrgIdent::Named(entity_name) => IndexValue::new_single(entity_name),
      OrgIdent::UserOrg(u) => IndexValue::new_single(format!("user-{}", u)),
    }
  }
}
