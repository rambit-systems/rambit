use std::sync::Arc;

use grid_state::NodeMeta;
use http::header::CONTENT_LENGTH;
use metrics_domain::{
  MetricsService,
  metrics_types::{
    UtcDateTime,
    http::{EventDetails, HttpEvent, ResponseDetails},
  },
};
use tower_http::trace::{DefaultOnResponse, OnResponse};

#[derive(Clone, Debug)]
pub struct MetricReporterOnResponse {
  inner:          DefaultOnResponse,
  metrics_domain: MetricsService,
  node_meta:      Arc<NodeMeta>,
}

impl MetricReporterOnResponse {
  pub fn new(
    inner: DefaultOnResponse,
    metrics_domain: MetricsService,
    node_meta: Arc<NodeMeta>,
  ) -> Self {
    Self {
      inner,
      metrics_domain,
      node_meta,
    }
  }
}

impl<B> OnResponse<B> for MetricReporterOnResponse {
  fn on_response(
    self,
    response: &http::Response<B>,
    latency: std::time::Duration,
    span: &tracing::Span,
  ) {
    self.inner.on_response(response, latency, span);

    let Some(request_id) =
      super::utils::extract_request_id_from_extensions(response.extensions())
    else {
      return;
    };

    let response_size = response.headers().get(CONTENT_LENGTH).and_then(|v| {
      match v.to_str().map(|v| v.parse::<u64>()) {
        Ok(Ok(size)) => Some(size),
        Ok(Err(_)) => {
          tracing::warn!("failed to parse Content-Length header as byte count");
          None
        }
        Err(_) => {
          tracing::warn!("header Content-Length contained non-ASCII");
          None
        }
      }
    });

    let event = HttpEvent {
      timestamp: UtcDateTime::now(),
      request_id,
      service_name: "grid".to_owned(),
      environment: self.node_meta.environment.clone(),
      host: self.node_meta.host_name.clone(),
      details: EventDetails::Response(ResponseDetails {
        status_code: response.status().as_u16(),
        latency,
        response_size_bytes: response_size,
      }),
    };

    let fut = async move {
      self.metrics_domain.send_event(event).await;
    };
    tokio::spawn(fut);
  }
}
