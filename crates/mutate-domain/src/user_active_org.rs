use db::DatabaseError;
use models::{Org, RecordId, User};

use crate::MutationService;

/// An error that occurs while updating a user's active org.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum UpdateActiveOrgError {
  /// Indicates that an error occurred with the database.
  #[error("Database error: {0}")]
  DatabaseError(#[from] DatabaseError),
  /// Indicates that the user does not exist.
  #[error("Failed to find user: {0}")]
  MissingUser(RecordId<User>),
  /// Indicates that the org supplied could not be switched to.
  #[error("Failed to switch to org: {0}")]
  InvalidOrg(RecordId<Org>),
}

impl MutationService {
  /// Switch a [`User`]'s active org.
  #[tracing::instrument(skip(self))]
  pub async fn switch_active_org(
    &self,
    user: RecordId<User>,
    new_active_org: RecordId<Org>,
  ) -> Result<RecordId<Org>, UpdateActiveOrgError> {
    let user = self
      .user_repo
      .get(user)
      .await?
      .ok_or(UpdateActiveOrgError::MissingUser(user))?;

    let new_index = user
      .iter_orgs()
      .position(|o| o == new_active_org)
      .ok_or(UpdateActiveOrgError::InvalidOrg(new_active_org))?;

    self
      .user_repo
      .update(&User {
        active_org_index: new_index as _,
        ..user
      })
      .await?;

    Ok(new_active_org)
  }
}
