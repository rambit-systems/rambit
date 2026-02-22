use axum::extract::Request;
use domain::models::ulid::Ulid;
use http::HeaderValue;
use tower_http::request_id::{MakeRequestId, RequestId};

#[derive(Clone, Copy, Default)]
pub struct MakeRequestUlid;

impl MakeRequestId for MakeRequestUlid {
  fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
    let ulid = Ulid::new();
    let request_id = HeaderValue::from_str(&ulid.to_string())
      .expect("failed to convert ULID to header value");
    Some(RequestId::new(request_id))
  }
}
