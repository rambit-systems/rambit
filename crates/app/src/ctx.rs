use std::sync::Arc;

use auth_domain::AuthSession;
use axum::{
  extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
  http::request::Parts,
};
use columbo::{SuspendedResponse, SuspenseContext};
use domain::DomainService;
use grid_state::AppState;

use crate::{
  extractors::AuthenticatedState, internal_error::InternalErrorRejection,
};

#[derive(Clone)]
pub struct Ctx(Arc<CtxInner>);

impl Ctx {
  pub fn state(&self) -> &AppState { &self.0.app_state }

  pub fn suspend<F, Fut, M>(
    &self,
    f: F,
    placeholder: impl Into<columbo::Html>,
  ) -> columbo::Suspense
  where
    F: FnOnce(Ctx) -> Fut,
    Fut: Future<Output = M> + Send + 'static,
    M: Into<columbo::Html> + 'static,
  {
    let fut = f(self.clone());
    self.0.suspense_ctx.suspend(fut, placeholder)
  }

  pub fn auth_state(&self) -> Option<AuthenticatedState> {
    self.0.authenticated_state.clone()
  }

  pub fn auth_session(&self) -> AuthSession { self.0.auth_session.clone() }
}

struct CtxInner {
  app_state:           AppState,
  suspense_ctx:        SuspenseContext,
  authenticated_state: Option<AuthenticatedState>,
  auth_session:        AuthSession,
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

    let auth_session = AuthSession::from_request_parts(parts, state)
      .await
      .map_err(|(s, e)| {
        miette::miette!(
          "failed to extract AuthSession with status code {s} and error \
           message: {e}"
        )
      })?;

    let ctx_inner = CtxInner {
      app_state,
      suspense_ctx,
      authenticated_state,
      auth_session,
    };

    Ok(ResponseSeed(Ctx(Arc::new(ctx_inner)), resp))
  }
}
