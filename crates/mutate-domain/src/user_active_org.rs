use db::{FetchModelError, PatchModelError};
use models::{Org, User, dvf::RecordId};

use crate::MutationService;

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

impl MutationService {
  /// Switch a [`User`]'s active org.
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
}
