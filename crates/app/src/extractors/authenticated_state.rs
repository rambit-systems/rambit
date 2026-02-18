use auth_domain::AuthSession;
use axum::{
  extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
  http::request::Parts,
};
use domain::{DomainService, models::AuthUser};
use miette::IntoDiagnostic;

use crate::{
  extractors::PathRequestedOrgId, hooks::OrgHook,
  internal_error::InternalErrorRejection,
};

/// State data regarding an authenticated user.
#[derive(Clone)]
pub struct AuthenticatedState {
  /// The authenticated user.
  pub auth_user:     AuthUser,
  /// The authenticated user's active org.
  pub active_org:    OrgHook,
  /// The org that the user requested with the route's `org` param. Option will
  /// be `None` if user does not belong to the requested org.
  pub requested_org: Option<OrgHook>,
}

impl<S> OptionalFromRequestParts<S> for AuthenticatedState
where
  S: Send + Sync,
  DomainService: FromRef<S>,
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
    let domain_service = DomainService::from_ref(state);
    let meta = domain_service.meta();
    let active_org_id = auth_user.active_org();
    tracing::error!(%active_org_id, "fetching active org for user");
    let active_org = meta
      .fetch_org_by_id(active_org_id)
      .await
      .inspect_err(|e| {
        tracing::error!(
          "failed to fetch user's active org for request state: {e}"
        );
      })
      .into_diagnostic()?
      .ok_or(miette::miette!(
        "user {user} authenticated but active org {active_org_id} doesn't \
         exist",
        user = auth_user.id
      ))?;
    let active_org = OrgHook::new(active_org, auth_user.clone());

    let requested_org_id = PathRequestedOrgId::from_request_parts(parts, state)
      .await
      .expect("failed to extract path");
    let requested_org = if let Some(requested_org_id) = requested_org_id {
      meta
        .fetch_org_by_id(requested_org_id.0)
        .await
        .inspect_err(|e| {
          tracing::error!(
            "failed to fetch user's requested org for request state: {e}"
          );
        })
        .into_diagnostic()?
        .map(|ro| OrgHook::new(ro, auth_user.clone()))
    } else {
      None
    };
    // return None if user is not a part of the org they're requesting.
    let requested_org =
      requested_org.filter(|ro| auth_user.belongs_to_org(ro.id()));

    Ok(Some(AuthenticatedState {
      auth_user,
      active_org,
      requested_org,
    }))
  }
}
