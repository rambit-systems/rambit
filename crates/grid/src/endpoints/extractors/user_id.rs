use std::str::FromStr;

use axum::{
  extract::FromRequestParts,
  http::{HeaderName, StatusCode},
  response::{IntoResponse, Response},
};
use prime_domain::models::{User, dvf::RecordId};

const USER_ID_HEADER_NAME: HeaderName = HeaderName::from_static("x-user-id");

pub struct UserIdExtractor(pub RecordId<User>);

impl<S: Sync> FromRequestParts<S> for UserIdExtractor {
  type Rejection = UserIdRejection;

  async fn from_request_parts(
    parts: &mut axum::http::request::Parts,
    _state: &S,
  ) -> Result<Self, Self::Rejection> {
    let Some(value) = parts.headers.get(USER_ID_HEADER_NAME) else {
      return Err(UserIdRejection(
        StatusCode::UNAUTHORIZED,
        format!("`{USER_ID_HEADER_NAME}` header missing"),
      ));
    };
    let Ok(value) = value.to_str() else {
      return Err(UserIdRejection(
        StatusCode::BAD_REQUEST,
        format!("`{USER_ID_HEADER_NAME}` header is not ASCII"),
      ));
    };
    let Ok(id) = RecordId::<User>::from_str(value) else {
      return Err(UserIdRejection(
        StatusCode::BAD_REQUEST,
        format!("`{USER_ID_HEADER_NAME}` header malformed: `{value}`"),
      ));
    };

    Ok(Self(id))
  }
}

pub struct UserIdRejection(StatusCode, String);

impl IntoResponse for UserIdRejection {
  fn into_response(self) -> Response { (self.0, self.1).into_response() }
}
