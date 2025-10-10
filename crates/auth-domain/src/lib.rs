//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

mod errors;
#[cfg(test)]
mod tests;

use axum_login::AuthUser as AxumLoginAuthUser;
pub use axum_login::AuthnBackend;
use db::{Database, FetchModelByIndexError, FetchModelError, kv::LaxSlug};
use miette::{IntoDiagnostic, miette};
use models::{
  AuthUser, Org, OrgIdent, User, UserAuthCredentials,
  UserSubmittedAuthCredentials, UserUniqueIndexSelector,
  dvf::{EitherSlug, EmailAddress, HumanName},
  model::RecordId,
};

pub use self::errors::*;

/// The authentication session type.
pub type AuthSession = axum_login::AuthSession<AuthDomainService>;

/// A dynamic [`AuthDomainService`] trait object.
#[derive(Clone, Debug)]
pub struct AuthDomainService {
  org_repo:  Database<Org>,
  user_repo: Database<User>,
}

impl AuthDomainService {
  /// Creates a new [`AuthDomainService`].
  #[must_use]
  pub fn new(org_repo: Database<Org>, user_repo: Database<User>) -> Self {
    Self {
      org_repo,
      user_repo,
    }
  }
}

impl AuthDomainService {
  /// Fetch a [`User`] by ID.
  async fn fetch_user_by_id(
    &self,
    id: RecordId<User>,
  ) -> Result<Option<User>, FetchModelError> {
    self.user_repo.fetch_model_by_id(id).await
  }

  /// Fetch a [`User`] by [`EmailAddress`](EmailAddress).
  async fn fetch_user_by_email(
    &self,
    email: EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError> {
    self
      .user_repo
      .fetch_model_by_unique_index(
        UserUniqueIndexSelector::Email,
        EitherSlug::Lax(LaxSlug::new(email.as_ref())),
      )
      .await
  }

  /// Switch a [`User`]'s active org.
  pub async fn switch_active_org(
    &self,
    user: RecordId<User>,
    new_active_org: RecordId<Org>,
  ) -> Result<RecordId<Org>, errors::UpdateActiveOrgError> {
    let user = self
      .user_repo
      .fetch_model_by_id(user)
      .await?
      .ok_or(errors::UpdateActiveOrgError::MissingUser(user))?;

    let new_index = user
      .iter_orgs()
      .position(|o| o == new_active_org)
      .ok_or(errors::UpdateActiveOrgError::InvalidOrg(new_active_org))?;

    self
      .user_repo
      .patch_model(user.id, User {
        active_org_index: new_index as _,
        ..user
      })
      .await?;

    Ok(new_active_org)
  }

  /// Sign up a [`User`].
  pub async fn user_signup(
    &self,
    name: HumanName,
    email: EmailAddress,
    auth: UserSubmittedAuthCredentials,
  ) -> Result<User, errors::CreateUserError> {
    use argon2::PasswordHasher;

    if self.fetch_user_by_email(email.clone()).await?.is_some() {
      return Err(errors::CreateUserError::EmailAlreadyUsed(email));
    }

    let auth: UserAuthCredentials = match auth {
      UserSubmittedAuthCredentials::Password { password } => {
        let salt = argon2::password_hash::SaltString::generate(
          &mut argon2::password_hash::rand_core::OsRng,
        );
        let argon = argon2::Argon2::default();
        let password_hash = models::PasswordHash(
          argon
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
              errors::CreateUserError::PasswordHashing(miette!(
                "failed to hash password: {e}"
              ))
            })?
            .to_string(),
        );

        UserAuthCredentials::Password { password_hash }
      }
    };

    let user_id = RecordId::new();

    let org = Org {
      id:        RecordId::new(),
      org_ident: OrgIdent::UserOrg(user_id),
    };

    let user = User {
      id: user_id,
      personal_org: org.id,
      orgs: Vec::new(),
      name: name.clone(),
      name_abbr: User::abbreviate_name(name),
      email,
      auth,
      active_org_index: 0,
    };

    self
      .org_repo
      .create_model(org)
      .await
      .into_diagnostic()
      .map_err(errors::CreateUserError::CreateError)?;

    self
      .user_repo
      .create_model(user)
      .await
      .into_diagnostic()
      .map_err(errors::CreateUserError::CreateError)
  }

  /// Authenticate a [`User`].
  pub async fn user_authenticate(
    &self,
    email: EmailAddress,
    creds: UserSubmittedAuthCredentials,
  ) -> Result<Option<User>, errors::AuthenticationError> {
    use argon2::PasswordVerifier;

    let Some(user) = self.fetch_user_by_email(email).await? else {
      return Ok(None);
    };

    match (creds, user.auth.clone()) {
      (
        UserSubmittedAuthCredentials::Password { password, .. },
        UserAuthCredentials::Password { password_hash, .. },
      ) => {
        let password_hash = argon2::PasswordHash::new(&password_hash.0)
          .map_err(|e| {
            errors::AuthenticationError::PasswordHashing(miette!(
              "failed to parse password hash: {e}"
            ))
          })?;

        let argon = argon2::Argon2::default();
        let correct =
          (match argon.verify_password(password.as_bytes(), &password_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e),
          })
          .map_err(|e| {
            errors::AuthenticationError::PasswordHashing(miette!(
              "failed to verify password against hash: {e}"
            ))
          })?;

        Ok(correct.then_some(user))
      }
    }
  }
}

impl AuthnBackend for AuthDomainService {
  type Credentials = (EmailAddress, UserSubmittedAuthCredentials);
  type Error = errors::AuthenticationError;
  type User = AuthUser;

  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    self
      .user_authenticate(creds.0, creds.1)
      .await
      .map(|u| u.map(Into::into))
  }

  async fn get_user(
    &self,
    id: &<Self::User as AxumLoginAuthUser>::Id,
  ) -> Result<Option<Self::User>, Self::Error> {
    self
      .fetch_user_by_id(*id)
      .await
      .map(|u| u.map(Into::into))
      .map_err(Into::into)
  }
}
