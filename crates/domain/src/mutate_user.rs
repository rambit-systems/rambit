//! User mutation logic.

use db::{FetchModelError, PatchModelError};
use models::{Org, User, dvf::RecordId};

use crate::DomainService;

/// The error enum for the
/// [`add_org_to_user`](DomainService::add_org_to_user).
#[derive(thiserror::Error, Debug)]
pub enum AddOrgToUserError {
  /// The user does not exist.
  #[error("The given user does not exist: {0}")]
  UserDoesNotExist(RecordId<User>),
  /// The org does not exist
  #[error("The given org does not exist: {0}")]
  OrgDoesNotExist(RecordId<Org>),
  /// The org is a personal org, which does not belong to the user.
  #[error(
    "The given org is a personal org, to which the user cannot be added: {0}"
  )]
  PersonalOrg(RecordId<Org>),
  /// The action has already been completed.
  #[error("This action has already been completed")]
  Idempotency,
  /// A fetch action failed.
  #[error("Internal error: failed to fetch model: {0}")]
  InternalFetchError(FetchModelError),
  /// A patch action failed.
  #[error("Internal error: failed to patch model: {0}")]
  InternalPatchError(PatchModelError),
}

impl DomainService {
  /// Adds an [`Org`] to a [`User`]'s org list.
  #[tracing::instrument(skip(self))]
  pub async fn add_org_to_user(
    &self,
    user: RecordId<User>,
    org: RecordId<Org>,
  ) -> Result<(), AddOrgToUserError> {
    let user = self
      .meta
      .fetch_user_by_id(user)
      .await
      .map_err(AddOrgToUserError::InternalFetchError)?
      .ok_or(AddOrgToUserError::UserDoesNotExist(user))?;

    if user.belongs_to_org(org) {
      return Err(AddOrgToUserError::Idempotency);
    }

    let org = self
      .meta
      .fetch_org_by_id(org)
      .await
      .map_err(AddOrgToUserError::InternalFetchError)?
      .ok_or(AddOrgToUserError::OrgDoesNotExist(org))?;

    if matches!(org.org_ident, models::OrgIdent::UserOrg(_)) {
      return Err(AddOrgToUserError::PersonalOrg(org.id));
    }

    let new_user = User {
      orgs: user.orgs.iter().copied().chain(Some(org.id)).collect(),
      ..user
    };

    self
      .mutate
      .patch_user(new_user)
      .await
      .map_err(AddOrgToUserError::InternalPatchError)?;

    Ok(())
  }
}
