use domain::models::model::Ulid;
use http::HeaderValue;
use tower_http::request_id::{MakeRequestId, RequestId};

#[derive(Clone, Default)]
pub struct MakeUlidRequestId;

impl MakeRequestId for MakeUlidRequestId {
  fn make_request_id<B>(
    &mut self,
    _request: &http::Request<B>,
  ) -> Option<tower_http::request_id::RequestId> {
    Some(RequestId::new(
      HeaderValue::from_str(&Ulid::new().to_string())
        .expect("failed to convert ULID to header value"),
    ))
  }
}
