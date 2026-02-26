use std::sync::Arc;

use auth_domain::AuthSession;
use axum::{
  body::Body,
  extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
  http::{Response, request::Parts},
  response::IntoResponse,
};
use columbo::{SuspendedResponse, SuspenseContext};
use domain::DomainService;
use grid_state::AppState;

use crate::{
  extractors::AuthenticatedState, internal_error::InternalErrorRejection,
  pages::util_pages::unauthorized_page,
};

/// Ctx marker type for collecting auth info.
pub struct MaybeAuth(Option<AuthenticatedState>);
/// Ctx marker type for requiring authentication.
pub struct RequireAuth(AuthenticatedState);

/// The context struct that every handler and most components use.
pub struct Ctx<A = MaybeAuth>(Arc<CtxTyped<A>>);

impl<A> Clone for Ctx<A> {
  fn clone(&self) -> Self { Self(self.0.clone()) }
}

struct CtxTyped<A> {
  shared: Arc<CtxShared>,
  auth:   A,
}

struct CtxShared {
  app_state:    AppState,
  suspense_ctx: SuspenseContext,
  auth_session: AuthSession,
}

/// The extractor that begins every page. Automatically starts columbo context.
pub struct ResponseSeed<Auth = MaybeAuth>(pub Ctx<Auth>, pub SuspendedResponse);

impl<Auth> Ctx<Auth> {
  /// Returns the [`AppState`].
  pub fn state(&self) -> &AppState { &self.0.shared.app_state }

  /// Returns the [`AuthSession`].
  pub fn auth_session(&self) -> AuthSession {
    self.0.shared.auth_session.clone()
  }

  /// Suspends a future with columbo.
  pub fn suspend<F, Fut, M>(
    &self,
    f: F,
    placeholder: impl Into<columbo::Html>,
  ) -> columbo::Suspense
  where
    Auth: Send + 'static,
    F: FnOnce(Ctx<Auth>) -> Fut,
    Fut: Future<Output = M> + Send + 'static,
    M: Into<columbo::Html> + 'static,
  {
    let fut = f(self.clone());
    self.0.shared.suspense_ctx.suspend(fut, placeholder)
  }
}

impl Ctx<MaybeAuth> {
  /// Returns the auth state if the user is logged in.
  pub fn auth_state(&self) -> Option<AuthenticatedState> {
    self.0.auth.0.clone()
  }
}

impl Ctx<RequireAuth> {
  /// Returns the logged-in user's auth state.
  pub fn auth_state(&self) -> AuthenticatedState { self.0.auth.0.clone() }

  /// Converts to a `Ctx<MaybeAuth>` for use with components that accept
  /// either authenticated or unauthenticated contexts.
  pub fn into_maybe_auth(self) -> Ctx<MaybeAuth> {
    Ctx(Arc::new(CtxTyped {
      shared: self.0.shared.clone(),
      auth:   MaybeAuth(Some(self.0.auth.0.clone())),
    }))
  }
}

impl From<Ctx<RequireAuth>> for Ctx<MaybeAuth> {
  fn from(value: Ctx<RequireAuth>) -> Self { value.into_maybe_auth() }
}

impl<S> FromRequestParts<S> for ResponseSeed<MaybeAuth>
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

    let shared = Arc::new(CtxShared {
      app_state,
      suspense_ctx,
      auth_session,
    });

    let ctx = Ctx(Arc::new(CtxTyped {
      shared,
      auth: MaybeAuth(authenticated_state),
    }));

    Ok(ResponseSeed(ctx, resp))
  }
}

impl<S> FromRequestParts<S> for ResponseSeed<RequireAuth>
where
  S: Send + Sync,
  AppState: FromRef<S>,
  DomainService: FromRef<AppState>,
{
  type Rejection = Response<Body>;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    let ResponseSeed(maybe_ctx, resp) =
      <ResponseSeed<MaybeAuth> as FromRequestParts<S>>::from_request_parts(
        parts, state,
      )
      .await
      .map_err(|e| e.into_response())?;

    let require_auth = match maybe_ctx.auth_state() {
      Some(auth_state) => RequireAuth(auth_state),
      None => {
        let response = resp
          .into_stream(unauthorized_page(maybe_ctx))
          .into_response();
        return Err(response);
      }
    };

    let ctx = Ctx(Arc::new(CtxTyped {
      shared: maybe_ctx.0.shared.clone(),
      auth:   require_auth,
    }));

    Ok(ResponseSeed(ctx, resp))
  }
}
