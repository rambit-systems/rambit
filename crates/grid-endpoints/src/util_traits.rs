use axum::response::{IntoResponse, Response};
use http::StatusCode;

/// A trait for obfuscating internal errors for public responses.
pub trait InternalError<R> {
  /// Obfuscates an internal error for a public response.
  fn internal(self, desc: &str) -> R;
}

impl<E: std::fmt::Debug> InternalError<Response> for E {
  fn internal(self, desc: &str) -> Response {
    tracing::error!("{desc}: {self:#?}");
    (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
  }
}
