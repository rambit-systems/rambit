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
use models::AuthUser;

use crate::{
  extractors::PathRequestedOrgId, hooks::OrgUrlHook,
  internal_error::InternalErrorRejection, pages::util_pages::unauthorized_page,
};

/// Ctx marker type for collecting auth info.
#[derive(Clone)]
pub struct MaybeAuth(Option<RequireAuth>);

/// Ctx marker type for requiring authentication.
#[derive(Clone)]
pub struct RequireAuth {
  auth_user:           AuthUser,
  active_org_url_hook: OrgUrlHook,
  requested_org:       Option<OrgUrlHook>,
}

/// Ctx marker type for requiring authentication and a requested org.
#[derive(Clone)]
pub struct RequireRequestedOrg {
  auth_user:           AuthUser,
  active_org_url_hook: OrgUrlHook,
  requested_org:       OrgUrlHook,
}

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
  /// Returns the [`AuthUser`], if it's populated.
  pub fn auth_user(&self) -> Option<AuthUser> {
    self.0.auth.0.as_ref().map(|ra| ra.auth_user.clone())
  }

  /// Returns a [`OrgUrlHook`] of the user's active org.
  pub fn active_org_url_hook(&self) -> Option<OrgUrlHook> {
    self
      .0
      .auth
      .0
      .as_ref()
      .map(|ra| ra.active_org_url_hook.clone())
  }

  /// Converts to a `Ctx<Required>` for use with components that an
  /// authenticated context.
  pub fn into_require_auth(self) -> Option<Ctx<RequireAuth>> {
    Some(Ctx(Arc::new(CtxTyped {
      shared: self.0.shared.clone(),
      auth:   self.0.auth.0.clone()?,
    })))
  }
}

impl Ctx<RequireAuth> {
  /// Returns the [`AuthUser`].
  pub fn auth_user(&self) -> AuthUser { self.0.auth.auth_user.clone() }

  /// Returns a [`OrgUrlHook`] of the user's active org.
  pub fn active_org_url_hook(&self) -> OrgUrlHook {
    self.0.auth.active_org_url_hook.clone()
  }

  /// Returns a [`OrgUrlHook`] of the requested org, if it exists.
  pub fn requested_org_url_hook(&self) -> Option<OrgUrlHook> {
    self.0.auth.requested_org.clone()
  }

  /// Converts to a `Ctx<MaybeAuth>` for use with components that accept
  /// either authenticated or unauthenticated contexts.
  pub fn into_maybe_auth(self) -> Ctx<MaybeAuth> {
    Ctx(Arc::new(CtxTyped {
      shared: self.0.shared.clone(),
      auth:   MaybeAuth(Some(self.0.auth.clone())),
    }))
  }
}

impl Ctx<RequireRequestedOrg> {
  /// Returns the [`AuthUser`].
  pub fn auth_user(&self) -> AuthUser { self.0.auth.auth_user.clone() }

  /// Returns a [`OrgUrlHook`] of the user's active org.
  pub fn active_org_url_hook(&self) -> OrgUrlHook {
    self.0.auth.active_org_url_hook.clone()
  }

  /// Returns a [`OrgUrlHook`] of the requested org.
  pub fn requested_org_url_hook(&self) -> OrgUrlHook {
    self.0.auth.requested_org.clone()
  }

  /// Converts to a `Ctx<RequireAuth>`.
  pub fn into_require_auth(self) -> Ctx<RequireAuth> {
    Ctx(Arc::new(CtxTyped {
      shared: self.0.shared.clone(),
      auth:   RequireAuth {
        auth_user:           self.0.auth.auth_user.clone(),
        active_org_url_hook: self.0.auth.active_org_url_hook.clone(),
        requested_org:       Some(self.0.auth.requested_org.clone()),
      },
    }))
  }

  /// Converts to a `Ctx<MaybeAuth>`.
  pub fn into_maybe_auth(self) -> Ctx<MaybeAuth> {
    self.into_require_auth().into_maybe_auth()
  }
}

impl From<Ctx<RequireRequestedOrg>> for Ctx<RequireAuth> {
  fn from(value: Ctx<RequireRequestedOrg>) -> Self { value.into_require_auth() }
}

impl From<Ctx<RequireRequestedOrg>> for Ctx<MaybeAuth> {
  fn from(value: Ctx<RequireRequestedOrg>) -> Self { value.into_maybe_auth() }
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

    let auth_session = AuthSession::from_request_parts(parts, state)
      .await
      .map_err(|(s, e)| {
        miette::miette!(
          "failed to extract AuthSession with status code {s} and error \
           message: {e}"
        )
      })?;

    let auth = match auth_session.user.clone() {
      Some(au) => {
        let requested_org =
          PathRequestedOrgId::from_request_parts(parts, state)
            .await
            .expect("failed to extract path")
            .map(|ro| ro.0)
            .filter(|ro| au.belongs_to_org(*ro))
            .map(OrgUrlHook::new);

        MaybeAuth(Some(RequireAuth {
          auth_user: au.clone(),
          active_org_url_hook: OrgUrlHook::new(au.active_org()),
          requested_org,
        }))
      }
      None => MaybeAuth(None),
    };

    let shared = Arc::new(CtxShared {
      app_state,
      suspense_ctx,
      auth_session,
    });

    let ctx = Ctx(Arc::new(CtxTyped { shared, auth }));

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

    let require_auth = match maybe_ctx.0.auth.0.clone() {
      Some(ra) => ra,
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

impl<S> FromRequestParts<S> for ResponseSeed<RequireRequestedOrg>
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
    let ResponseSeed(require_ctx, resp) =
      <ResponseSeed<RequireAuth> as FromRequestParts<S>>::from_request_parts(
        parts, state,
      )
      .await?;

    let requested_org = match require_ctx.0.auth.requested_org.clone() {
      Some(org) => org,
      None => {
        // No requested org in the path — redirect or show error.
        let response = resp
          .into_stream(unauthorized_page(require_ctx.into_maybe_auth()))
          .into_response();
        return Err(response);
      }
    };

    let ctx = Ctx(Arc::new(CtxTyped {
      shared: require_ctx.0.shared.clone(),
      auth:   RequireRequestedOrg {
        auth_user: require_ctx.0.auth.auth_user.clone(),
        active_org_url_hook: require_ctx.0.auth.active_org_url_hook.clone(),
        requested_org,
      },
    }));

    Ok(ResponseSeed(ctx, resp))
  }
}
