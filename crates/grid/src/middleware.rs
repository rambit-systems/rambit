pub mod cache_on_success;
pub mod compression_predicate;
pub mod make_ulid_request_id;
pub mod on_request_metric_reporter;
pub mod on_response_metric_reporter;

mod utils {
  use domain::models::model::Ulid;
  use http::Extensions;
  use tower_http::request_id::RequestId;

  pub(super) fn extract_request_id_from_extensions(
    ext: &Extensions,
  ) -> Option<Ulid> {
    let Some(request_id) = ext.get::<RequestId>() else {
      tracing::warn!("could not extract request ID from extensions");
      return None;
    };
    let Ok(request_id) = request_id.header_value().to_str() else {
      tracing::warn!("invalid characters in request ID");
      return None;
    };
    let Ok(request_id) = request_id.parse() else {
      tracing::warn!("failed to parse request ID as ULID");
      return None;
    };

    Some(request_id)
  }
}
