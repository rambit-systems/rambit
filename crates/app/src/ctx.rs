use std::{ops::Deref, sync::Arc};

use axum::{
  extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
  http::request::Parts,
};
use columbo::{SuspendedResponse, SuspenseContext};
use domain::DomainService;
use grid_state::AppState;
use models::AuthUser;

use crate::{
  extractors::AuthenticatedState, hooks::OrgHook,
  internal_error::InternalErrorRejection,
};

#[derive(Clone)]
pub struct Ctx(Arc<CtxInner>);

impl Ctx {
  pub fn state(&self) -> &AppState { &self.0.app_state }

  pub fn auth_user(&self) -> Option<AuthUser> {
    self
      .0
      .authenticated_state
      .as_ref()
      .map(|a| a.auth_user.clone())
  }

  pub fn active_org(&self) -> Option<OrgHook> {
    self
      .0
      .authenticated_state
      .as_ref()
      .map(|a| a.active_org.clone())
  }
}

impl Deref for Ctx {
  type Target = SuspenseContext;

  fn deref(&self) -> &Self::Target { &self.0.suspense_ctx }
}

struct CtxInner {
  app_state:           AppState,
  suspense_ctx:        SuspenseContext,
  authenticated_state: Option<AuthenticatedState>,
}

pub struct ResponseSeed(pub Ctx, pub SuspendedResponse);

impl<S> FromRequestParts<S> for ResponseSeed
where
  S: Send + Sync,
  AppState: FromRef<S>,
  DomainService: FromRef<AppState>,
{
  type Rejection = InternalErrorRejection;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    let (suspense_ctx, resp) = columbo::new();

    let app_state = AppState::from_ref(state);

    let authenticated_state =
      <AuthenticatedState as OptionalFromRequestParts<AppState>>::from_request_parts(
        parts, &app_state,
      )
      .await?;

    let ctx_inner = CtxInner {
      app_state,
      suspense_ctx,
      authenticated_state,
    };

    Ok(ResponseSeed(Ctx(Arc::new(ctx_inner)), resp))
  }
}
