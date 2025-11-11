use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub trait InternalError<R> {
  fn internal(self, desc: &str) -> R;
}

impl<E: std::fmt::Debug> InternalError<Response> for E {
  fn internal(self, desc: &str) -> Response {
    tracing::error!("{desc}: {self:#?}");
    (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
  }
}
