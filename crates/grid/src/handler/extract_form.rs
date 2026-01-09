use std::collections::HashMap;

use axum::{body::Body, extract::Request};
use http_body_util::BodyExt;
use tracing::debug;

/// This function will extract form data from a request and then reconstruct it
/// for ownership purposes, or leave it alone if it does not have a form body.
pub async fn extract_form_data_from_body(
  mut request: Request<Body>,
) -> (Request<Body>, Option<HashMap<String, String>>) {
  let content_type = request
    .headers()
    .get(axum::http::header::CONTENT_TYPE)
    .and_then(|v| v.to_str().ok());

  match content_type {
    Some(ct) if ct.starts_with("application/x-www-form-urlencoded") => {
      // Collect the entire body into bytes
      let (parts, body) = request.into_parts();
      let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) => {
          debug!("failed to read body: {}", e);
          // Reconstruct request with empty body
          request = Request::from_parts(parts, Body::empty());
          return (request, None);
        }
      };

      // Parse form data from the bytes
      let form_result =
        serde_urlencoded::from_bytes::<HashMap<String, String>>(&bytes);

      // Reconstruct the request with the buffered body
      request = Request::from_parts(parts, Body::from(bytes));

      match form_result {
        Ok(data) => (request, Some(data)),
        Err(e) => {
          debug!("failed to parse form data: {}", e);
          (request, None)
        }
      }
    }
    _ => (request, None),
  }
}
