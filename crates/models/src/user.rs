use std::{
  fmt,
  hash::{self, Hash, Hasher},
  iter::once,
};

use dvf::{EitherSlug, EmailAddress, HumanName, LaxSlug, RecordId};
use model::{Model, SlugFieldGetter};
use serde::{Deserialize, Serialize};

use crate::Org;

/// A user.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  /// The user's ID.
  pub id:               RecordId<User>,
  /// The user's orgs, guaranteed to be at least one.
  pub orgs:             (RecordId<Org>, Vec<RecordId<Org>>),
  /// The user's name.
  pub name:             HumanName,
  /// The user's email address.
  pub email:            EmailAddress,
  /// The user's authentication secrets.
  pub auth:             UserAuthCredentials,
  /// The index of the [`Org`] that the user is currently operating as.
  pub active_org_index: u8,
}

impl User {
  /// Returns the hash of the user's authentication secrets.
  pub fn auth_hash(&self) -> u64 {
    let mut hasher = hash::DefaultHasher::new();
    self.auth.hash(&mut hasher);
    hasher.finish()
  }

  /// Generates the value of the unique [`User`] index `email`.
  pub fn unique_index_email(&self) -> Vec<EitherSlug> {
    vec![EitherSlug::Lax(LaxSlug::new(self.email.as_ref()))]
  }

  /// Returns an iterator of the user's orgs.
  pub fn iter_orgs(&self) -> impl Iterator<Item = RecordId<Org>> {
    once(self.orgs.0).chain(self.orgs.1.iter().copied())
  }

  /// Returns whether the user belongs to the given org.
  pub fn belongs_to_org(&self, org: RecordId<Org>) -> bool {
    self.orgs.0 == org || self.orgs.1.contains(&org)
  }
}

/// The unique index selector for [`User`]
#[derive(Debug, Clone, Copy)]
pub enum UserUniqueIndexSelector {
  /// The `email` index.
  Email,
}

impl fmt::Display for UserUniqueIndexSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      UserUniqueIndexSelector::Email => write!(f, "email"),
    }
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

impl Model for User {
  type IndexSelector = !;
  type UniqueIndexSelector = UserUniqueIndexSelector;

  const INDICES: &'static [(
    Self::IndexSelector,
    model::SlugFieldGetter<Self>,
  )] = &[];
  const TABLE_NAME: &'static str = "user";
  const UNIQUE_INDICES: &'static [(
    Self::UniqueIndexSelector,
    SlugFieldGetter<Self>,
  )] = &[(UserUniqueIndexSelector::Email, User::unique_index_email)];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}

/// An auth-centric view of a [`User`], able to be sent to the client.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthUser {
  /// The user's ID.
  pub id:               RecordId<User>,
  /// The user's orgs, guaranteed to be at least one.
  pub orgs:             (RecordId<Org>, Vec<RecordId<Org>>),
  /// The user's name.
  pub name:             HumanName,
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
      orgs: user.orgs,
      name: user.name,
      auth_hash_bytes,
      active_org_index: user.active_org_index,
    }
  }
}

impl AuthUser {
  /// Returns an iterator of the user's orgs.
  pub fn iter_orgs(&self) -> impl Iterator<Item = RecordId<Org>> {
    once(self.orgs.0).chain(self.orgs.1.iter().copied())
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
