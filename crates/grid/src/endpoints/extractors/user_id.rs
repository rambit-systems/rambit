use auth_domain::AuthSession;
use axum::{
  extract::{FromRequestParts, OptionalFromRequestParts},
  http::StatusCode,
};
use prime_domain::models::AuthUser;

/// Uses [`AuthSession`] to extract [`AuthUser`]. This extra indirection is here
/// so that we can extract user auth from multiple sources in the future.
pub struct UserAuthExtractor(pub AuthUser);

impl<S: Send + Sync> FromRequestParts<S> for UserAuthExtractor {
  type Rejection = (StatusCode, &'static str);

  async fn from_request_parts(
    parts: &mut http::request::Parts,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    // extract using the optional form and then throw an error on None
    <Self as OptionalFromRequestParts<S>>::from_request_parts(parts, state)
      .await?
      .ok_or((
        StatusCode::UNAUTHORIZED,
        "UNAUTHORIZED: session header missing",
      ))
  }
}

impl<S: Send + Sync> OptionalFromRequestParts<S> for UserAuthExtractor {
  type Rejection = (StatusCode, &'static str);

  async fn from_request_parts(
    parts: &mut http::request::Parts,
    state: &S,
  ) -> Result<Option<Self>, Self::Rejection> {
    // extract AuthSession straight from the request and pull the user field
    AuthSession::from_request_parts(parts, state)
      .await
      .map(|s| s.user.map(Self))
  }
}
