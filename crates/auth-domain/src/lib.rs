//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

mod errors;
#[cfg(test)]
mod tests;

use axum_login::AuthUser as AxumLoginAuthUser;
pub use axum_login::AuthnBackend;
use miette::{Context, IntoDiagnostic, miette};
use models::{
  AuthUser, Org, OrgIdent, User, UserAuthCredentials,
  UserSubmittedAuthCredentials,
  dvf::{EmailAddress, HumanName},
  model::RecordId,
};
pub use mutate_domain::UpdateActiveOrgError;

pub use self::errors::*;

/// The authentication session type.
pub type AuthSession = axum_login::AuthSession<AuthDomainService>;

/// A dynamic [`AuthDomainService`] trait object.
#[derive(Clone, Debug)]
pub struct AuthDomainService {
  meta:   meta_domain::MetaService,
  mutate: mutate_domain::MutationService,
}

impl AuthDomainService {
  /// Creates a new [`AuthDomainService`].
  #[must_use]
  pub fn new(
    meta: meta_domain::MetaService,
    mutate: mutate_domain::MutationService,
  ) -> Self {
    Self { meta, mutate }
  }
}

impl AuthDomainService {
  /// Switches the active org of a [`User`].
  pub async fn switch_active_org(
    &self,
    user: RecordId<User>,
    new_active_org: RecordId<Org>,
  ) -> Result<RecordId<Org>, UpdateActiveOrgError> {
    self.mutate.switch_active_org(user, new_active_org).await
  }

  /// Sign up a [`User`].
  #[tracing::instrument(skip(self))]
  pub async fn user_signup(
    &self,
    name: HumanName,
    email: EmailAddress,
    auth: UserSubmittedAuthCredentials,
  ) -> Result<User, errors::CreateUserError> {
    use argon2::PasswordHasher;

    if self
      .meta
      .fetch_user_by_email(email.clone())
      .await
      .into_diagnostic()
      .context("failed to check for conflicting user by email")
      .map_err(CreateUserError::InternalError)?
      .is_some()
    {
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
              CreateUserError::InternalError(miette!(
                "failed to parse password hash: {e}"
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
      .mutate
      .create_org(org)
      .await
      .into_diagnostic()
      .context("failed to create personal org for user")
      .map_err(errors::CreateUserError::InternalError)?;

    self
      .mutate
      .create_user(user.clone())
      .await
      .into_diagnostic()
      .context("failed to create user")
      .map_err(errors::CreateUserError::InternalError)?;

    Ok(user)
  }

  /// Authenticate a [`User`].
  #[tracing::instrument(skip(self))]
  pub async fn user_authenticate(
    &self,
    email: EmailAddress,
    creds: UserSubmittedAuthCredentials,
  ) -> Result<Option<User>, AuthenticationError> {
    use argon2::PasswordVerifier;

    let Some(user) = self
      .meta
      .fetch_user_by_email(email)
      .await
      .into_diagnostic()
      .context("failed to fetch user by email")
      .map_err(AuthenticationError)?
    else {
      return Ok(None);
    };

    match (creds, user.auth.clone()) {
      (
        UserSubmittedAuthCredentials::Password { password, .. },
        UserAuthCredentials::Password { password_hash, .. },
      ) => {
        let password_hash = argon2::PasswordHash::new(&password_hash.0)
          .map_err(|e| {
            AuthenticationError(miette!("failed to parse password hash: {e}"))
          })?;

        let argon = argon2::Argon2::default();
        let correct =
          (match argon.verify_password(password.as_bytes(), &password_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e),
          })
          .map_err(|e| {
            AuthenticationError(miette!(
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

  #[tracing::instrument(skip(self))]
  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    self
      .user_authenticate(creds.0, creds.1)
      .await
      .map(|u| u.map(Into::into))
  }

  #[tracing::instrument(skip(self))]
  async fn get_user(
    &self,
    id: &<Self::User as AxumLoginAuthUser>::Id,
  ) -> Result<Option<Self::User>, Self::Error> {
    self
      .meta
      .fetch_user_by_id(*id)
      .await
      .into_diagnostic()
      .map(|u| u.map(Into::into))
      .map_err(AuthenticationError)
  }
}
