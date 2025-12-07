//! Authentication methods and types.

use miette::{Context, IntoDiagnostic, miette};
use models::{
  EmailAddress, User, UserAuthCredentials, UserSubmittedAuthCredentials,
};

use crate::DomainService;

/// An error that occurs during user authentication.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("Internal error: {0}")]
pub struct AuthenticationError(pub miette::Report);

impl DomainService {
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
