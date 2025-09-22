//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

use axum_login::AuthUser as AxumLoginAuthUser;
pub use axum_login::AuthnBackend;
use db::{
  Database, FetchModelByIndexError, FetchModelError, PatchModelError,
  kv::LaxSlug,
};
use miette::{IntoDiagnostic, miette};
use models::{
  AuthUser, Org, OrgIdent, User, UserAuthCredentials,
  UserSubmittedAuthCredentials, UserUniqueIndexSelector,
  dvf::{EitherSlug, EmailAddress, HumanName},
  model::RecordId,
};
use tracing::instrument;

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

/// An error that occurs during user creation.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CreateUserError {
  /// Indicates that the user's email address is already in use.
  #[error("The email address is already in use: \"{0}\"")]
  EmailAlreadyUsed(EmailAddress),
  /// Indicates than an error occurred while hashing the password.
  #[error("Failed to hash password")]
  PasswordHashing(miette::Report),
  /// Indicates that an error occurred while creating the user.
  #[error("Failed to create the user")]
  CreateError(miette::Report),
  /// Indicates that an error occurred while fetching users by index.
  #[error("Failed to fetch users by index")]
  FetchByIndexError(#[from] FetchModelByIndexError),
}

/// An error that occurs during user authentication.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AuthenticationError {
  /// Indicates that an error occurred while fetching users.
  #[error("Failed to fetch user")]
  FetchError(#[from] FetchModelError),
  /// Indicates that an error occurred while fetching users by index.
  #[error("Failed to fetch user by index")]
  FetchByIndexError(#[from] FetchModelByIndexError),
  /// Indicates than an error occurred while hashing the password.
  #[error("Failed to hash password")]
  PasswordHashing(miette::Report),
}

/// An error that occurs while updating a user's active org.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum UpdateActiveOrgError {
  /// Indicates that an error occurred while fetching the user.
  #[error("Failed to fetch user")]
  FetchError(#[from] FetchModelError),
  /// Indicates that the user does not exist.
  #[error("Failed to find user: {0}")]
  MissingUser(RecordId<User>),
  /// Indicates that the org supplied could not be switched to.
  #[error("Failed to switch to org: {0}")]
  InvalidOrg(RecordId<Org>),
  /// Indicates that an error occurred while patching the user record.
  #[error("Failed to patch user")]
  PatchError(#[from] PatchModelError),
}

impl AuthDomainService {
  /// Fetch a [`User`] by ID.
  #[instrument(skip(self))]
  pub async fn fetch_user_by_id(
    &self,
    id: RecordId<User>,
  ) -> Result<Option<User>, FetchModelError> {
    self.user_repo.fetch_model_by_id(id).await
  }

  /// Fetch a [`User`] by [`EmailAddress`](EmailAddress).
  #[instrument(skip(self))]
  pub async fn fetch_user_by_email(
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
  #[instrument(skip(self))]
  pub async fn switch_active_org(
    &self,
    user: RecordId<User>,
    new_active_org: RecordId<Org>,
  ) -> Result<RecordId<Org>, UpdateActiveOrgError> {
    let user = self
      .user_repo
      .fetch_model_by_id(user)
      .await?
      .ok_or(UpdateActiveOrgError::MissingUser(user))?;

    let new_index = user
      .iter_orgs()
      .position(|o| o == new_active_org)
      .ok_or(UpdateActiveOrgError::InvalidOrg(new_active_org))?;

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
  #[instrument(skip(self))]
  pub async fn user_signup(
    &self,
    name: HumanName,
    email: EmailAddress,
    auth: UserSubmittedAuthCredentials,
  ) -> Result<User, CreateUserError> {
    use argon2::PasswordHasher;

    if self.fetch_user_by_email(email.clone()).await?.is_some() {
      return Err(CreateUserError::EmailAlreadyUsed(email));
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
              CreateUserError::PasswordHashing(miette!(
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
      name,
      email,
      auth,
      active_org_index: 0,
    };

    self
      .org_repo
      .create_model(org)
      .await
      .into_diagnostic()
      .map_err(CreateUserError::CreateError)?;

    self
      .user_repo
      .create_model(user)
      .await
      .into_diagnostic()
      .map_err(CreateUserError::CreateError)
  }

  /// Authenticate a [`User`].
  #[instrument(skip(self))]
  pub async fn user_authenticate(
    &self,
    email: EmailAddress,
    creds: UserSubmittedAuthCredentials,
  ) -> Result<Option<User>, AuthenticationError> {
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
            AuthenticationError::PasswordHashing(miette!(
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
            AuthenticationError::PasswordHashing(miette!(
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
  type Error = AuthenticationError;
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

#[cfg(test)]
mod tests {
  use models::dvf::{EmailAddress, HumanName};

  use super::*;

  #[tokio::test]
  async fn test_user_signup() {
    let org_repo = Database::new_mock();
    let user_repo = Database::new_mock();
    let service = AuthDomainService::new(org_repo, user_repo);

    let name = HumanName::try_new("Test User 1").unwrap();
    let email = EmailAddress::try_new("test@example.com").unwrap();
    let creds = UserSubmittedAuthCredentials::Password {
      password: "hunter42".to_string(),
    };
    let user = service
      .user_signup(name, email.clone(), creds.clone())
      .await
      .unwrap();
    assert_eq!(user.email, email);

    dbg!(&service);

    let name = HumanName::try_new("Test User 2").unwrap();
    let user2 = service
      .user_signup(name, email.clone(), creds.clone())
      .await;
    assert!(matches!(user2, Err(CreateUserError::EmailAlreadyUsed(_))));
  }

  #[tokio::test]
  async fn test_user_authenticate() {
    let org_repo = Database::new_mock();
    let user_repo = Database::new_mock();
    let service = AuthDomainService::new(org_repo, user_repo);

    let name = HumanName::try_new("Test User 1").unwrap();
    let email = EmailAddress::try_new("test@example.com").unwrap();
    let creds = UserSubmittedAuthCredentials::Password {
      password: "hunter42".to_string(),
    };
    let user = service
      .user_signup(name, email.clone(), creds.clone())
      .await
      .unwrap();
    assert_eq!(user.email, email);

    let auth_user = service
      .user_authenticate(email.clone(), creds)
      .await
      .unwrap();
    assert_eq!(auth_user, Some(user));

    let wrong_email = EmailAddress::try_new("untest@example.com").unwrap();
    let creds = UserSubmittedAuthCredentials::Password {
      password: "hunter42".to_string(),
    };
    let auth_user =
      service.user_authenticate(wrong_email, creds).await.unwrap();
    assert_eq!(auth_user, None);
  }
}
