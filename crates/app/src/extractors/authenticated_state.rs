use auth_domain::AuthSession;
use axum::{
  extract::{FromRequestParts, OptionalFromRequestParts},
  http::request::Parts,
};
use domain::models::AuthUser;

use crate::{
  extractors::PathRequestedOrgId, hooks::OrgUrlHook,
  internal_error::InternalErrorRejection,
};

/// State data regarding an authenticated user.
#[derive(Clone)]
pub struct AuthenticatedState {
  /// The authenticated user.
  auth_user:              AuthUser,
  /// The authenticated user's active org.
  active_org_url_hook:    OrgUrlHook,
  /// The org that the user requested with the route's `org` param. Option will
  /// be `None` if user does not belong to the requested org.
  requested_org_url_hook: Option<OrgUrlHook>,
}

impl AuthenticatedState {
  pub fn auth_user(&self) -> AuthUser { self.auth_user.clone() }

  pub fn active_org_url_hook(&self) -> OrgUrlHook {
    self.active_org_url_hook.clone()
  }

  pub fn requested_org_url_hook(&self) -> Option<OrgUrlHook> {
    self.requested_org_url_hook.clone()
  }
}

impl<S> OptionalFromRequestParts<S> for AuthenticatedState
where
  S: Send + Sync,
{
  type Rejection = InternalErrorRejection;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &S,
  ) -> Result<Option<Self>, Self::Rejection> {
    // extract the auth user, and bail if not authenticated
    let auth_session = AuthSession::from_request_parts(parts, state)
      .await
      .expect("could not extract AuthSession");
    let Some(auth_user) = auth_session.user else {
      return Ok(None);
    };

    // fetch the active org
    let active_org_id = auth_user.active_org();
    let active_org_url_hook = OrgUrlHook::new(active_org_id);

    let requested_org_id = PathRequestedOrgId::from_request_parts(parts, state)
      .await
      .expect("failed to extract path")
      .map(|ro| ro.0);

    // return None if user is not a part of the org they're requesting.
    let requested_org_url_hook = requested_org_id
      .filter(|ro| auth_user.belongs_to_org(*ro))
      .map(OrgUrlHook::new);

    Ok(Some(AuthenticatedState {
      auth_user,
      active_org_url_hook,
      requested_org_url_hook,
    }))
  }
}
