use std::str::FromStr;

use axum::{
  extract::{FromRequestParts, OptionalFromRequestParts},
  http::{HeaderName, StatusCode},
};
use prime_domain::models::{User, dvf::RecordId};

const USER_ID_HEADER_NAME: HeaderName = HeaderName::from_static("x-user-id");

pub struct UserIdExtractor(pub RecordId<User>);

impl<S: Sync> FromRequestParts<S> for UserIdExtractor {
  type Rejection = (StatusCode, String);

  async fn from_request_parts(
    parts: &mut axum::http::request::Parts,
    _state: &S,
  ) -> Result<Self, Self::Rejection> {
    let Some(value) = parts.headers.get(USER_ID_HEADER_NAME) else {
      return Err((
        StatusCode::UNAUTHORIZED,
        format!("`{USER_ID_HEADER_NAME}` header missing"),
      ));
    };
    let Ok(value) = value.to_str() else {
      return Err((
        StatusCode::BAD_REQUEST,
        format!("`{USER_ID_HEADER_NAME}` header is not ASCII"),
      ));
    };
    let Ok(id) = RecordId::<User>::from_str(value) else {
      return Err((
        StatusCode::BAD_REQUEST,
        format!("`{USER_ID_HEADER_NAME}` header malformed: `{value}`"),
      ));
    };

    Ok(Self(id))
  }
}

impl<S: Sync> OptionalFromRequestParts<S> for UserIdExtractor {
  type Rejection = ();

  async fn from_request_parts(
    parts: &mut axum::http::request::Parts,
    state: &S,
  ) -> Result<Option<Self>, Self::Rejection> {
    Ok(
      <UserIdExtractor as FromRequestParts<S>>::from_request_parts(
        parts, state,
      )
      .await
      .ok(),
    )
  }
}
