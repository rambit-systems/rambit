use std::{
  hash::{self, Hash, Hasher},
  iter::once,
};

use model::{IndexValue, Model, RecordId};
use model_types::{EmailAddress, HumanName, PaddleCustomerId};
use serde::{Deserialize, Serialize};

use crate::Org;

/// A user.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(
  table = "users",
  index(name = "org", extract =
    |m| m.iter_orgs().map(|r| IndexValue::new_single(r.to_string())).collect()
  ),
  index(name = "email", unique, extract = |m| vec![IndexValue::new_single(&m.email)])
)]
pub struct User {
  /// The user's ID.
  #[model(id)]
  pub id:               RecordId<User>,
  /// The user's personal org.
  pub personal_org:     RecordId<Org>,
  /// The user's named orgs.
  pub orgs:             Vec<RecordId<Org>>,
  /// The user's name.
  pub name:             HumanName,
  /// An abbreviated form of the user's name.
  pub name_abbr:        HumanName,
  /// The user's email address.
  pub email:            EmailAddress,
  /// The user's authentication secrets.
  pub auth:             UserAuthCredentials,
  /// The index of the [`Org`] that the user is currently operating as.
  pub active_org_index: u8,
  /// The customer ID for this org in Paddle.
  pub customer_id:      PaddleCustomerId,
}

impl User {
  /// Returns the hash of the user's authentication secrets.
  pub fn auth_hash(&self) -> u64 {
    let mut hasher = hash::DefaultHasher::new();
    self.auth.hash(&mut hasher);
    hasher.finish()
  }

  /// Returns an iterator of the user's orgs.
  pub fn iter_orgs(&self) -> impl Iterator<Item = RecordId<Org>> {
    once(self.personal_org).chain(self.orgs.iter().copied())
  }

  /// Returns whether the user belongs to the given org.
  pub fn belongs_to_org(&self, org: RecordId<Org>) -> bool {
    self.personal_org == org || self.orgs.contains(&org)
  }

  /// Helper fn that abbreviates a name.
  pub fn abbreviate_name(name: HumanName) -> HumanName {
    HumanName::try_new(
      name
        .to_string()
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .collect::<String>(),
    )
    .expect("failed to create name")
  }
}

/// A password hash.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct PasswordHash(pub String);

/// The user-submitted version of [`UserAuthCredentials`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserSubmittedAuthCredentials {
  /// Standard password.
  Password {
    /// The password used.
    password: String,
  },
}

/// The authentication method for a [`User`].
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum UserAuthCredentials {
  /// Standard password (hash).
  Password {
    /// The hash of the password used.
    password_hash: PasswordHash,
  },
}

/// An auth-centric view of a [`User`], able to be sent to the client.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthUser {
  /// The user's ID.
  pub id:               RecordId<User>,
  /// The user's personal org.
  pub personal_org:     RecordId<Org>,
  /// The user's named orgs.
  pub orgs:             Vec<RecordId<Org>>,
  /// The user's name.
  pub name:             HumanName,
  /// An abbreviated form of the user's name.
  pub name_abbr:        HumanName,
  /// The user's email address.
  pub email:            EmailAddress,
  /// The hash of the user's authentication secrets.
  pub auth_hash_bytes:  Box<[u8]>,
  /// The index of the [`Org`] that the user is currently operating as.
  pub active_org_index: u8,
}

impl From<User> for AuthUser {
  fn from(user: User) -> Self {
    let auth_hash_bytes =
      user.auth_hash().to_be_bytes().to_vec().into_boxed_slice();
    Self {
      id: user.id,
      personal_org: user.personal_org,
      orgs: user.orgs,
      name: user.name,
      name_abbr: user.name_abbr,
      email: user.email,
      auth_hash_bytes,
      active_org_index: user.active_org_index,
    }
  }
}

impl AuthUser {
  /// Returns an iterator of the user's orgs.
  pub fn iter_orgs(&self) -> impl Iterator<Item = RecordId<Org>> {
    once(self.personal_org).chain(self.orgs.iter().copied())
  }

  /// Returns whether the user belongs to the given org.
  pub fn belongs_to_org(&self, org: RecordId<Org>) -> bool {
    self.personal_org == org || self.orgs.contains(&org)
  }

  /// Returns the ID of the currently active org.
  pub fn active_org(&self) -> RecordId<Org> {
    self
      .iter_orgs()
      .nth(self.active_org_index as _)
      .expect("active_org_index is out of bounds")
  }
}

#[cfg(feature = "auth")]
mod auth {
  use axum_login::AuthUser as AxumLoginAuthUser;

  use super::AuthUser;

  impl AxumLoginAuthUser for AuthUser {
    type Id = super::RecordId<super::User>;

    fn id(&self) -> Self::Id { self.id }

    fn session_auth_hash(&self) -> &[u8] { &self.auth_hash_bytes }
  }
}
