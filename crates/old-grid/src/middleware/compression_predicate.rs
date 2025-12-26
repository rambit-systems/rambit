use http_body::Body;
use tower_http::compression::Predicate;

#[derive(Clone, Debug)]
pub struct NotForFailureStatus {}

impl NotForFailureStatus {
  pub fn new() -> Self { Self {} }
}

impl Predicate for NotForFailureStatus {
  fn should_compress<B>(&self, response: &http::Response<B>) -> bool
  where
    B: Body,
  {
    response.status().is_success()
  }
}
