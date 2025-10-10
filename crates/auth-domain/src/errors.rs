use db::{FetchModelByIndexError, FetchModelError, PatchModelError};
use models::{Org, User, dvf::EmailAddress, model::RecordId};

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
