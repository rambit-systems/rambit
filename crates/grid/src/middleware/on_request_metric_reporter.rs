use std::sync::Arc;

use http::header::{ORIGIN, USER_AGENT};
use metrics_domain::{
  MetricsService,
  metrics_types::{
    UtcDateTime,
    http::{EventDetails, HttpEvent, RequestDetails},
  },
};
use tower_http::trace::{DefaultOnRequest, OnRequest};

use crate::app_state::NodeMeta;

#[derive(Clone, Debug)]
pub struct MetricReporterOnRequest {
  inner:          DefaultOnRequest,
  metrics_domain: MetricsService,
  node_meta:      Arc<NodeMeta>,
}

impl MetricReporterOnRequest {
  pub fn new(
    inner: DefaultOnRequest,
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

impl<B> OnRequest<B> for MetricReporterOnRequest {
  fn on_request(&mut self, request: &http::Request<B>, span: &tracing::Span) {
    self.inner.on_request(request, span);

    let Some(request_id) =
      super::utils::extract_request_id_from_extensions(request.extensions())
    else {
      return;
    };

    let origin = request
      .headers()
      .get(ORIGIN)
      .and_then(|v| v.to_str().ok())
      .map(|s| s.to_owned());
    let user_agent = request
      .headers()
      .get(USER_AGENT)
      .and_then(|v| v.to_str().ok())
      .map(|s| s.to_owned());

    let event = HttpEvent {
      timestamp: UtcDateTime::now(),
      request_id,
      service_name: "grid".to_owned(),
      environment: self.node_meta.environment.clone(),
      host: self.node_meta.host_name.clone(),
      details: EventDetails::Request(RequestDetails {
        method: request.method().to_string(),
        path: request.uri().path().to_owned(),
        uri: request.uri().to_string(),
        origin,
        user_agent,
      }),
    };

    let fut = {
      let metrics_domain = self.metrics_domain.clone();
      async move {
        metrics_domain.clone().send_event(event).await;
      }
    };
    tokio::spawn(fut);
  }
}
