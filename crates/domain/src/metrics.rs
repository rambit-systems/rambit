use metrics_domain::metrics::Metric;

use crate::DomainService;

impl DomainService {
  /// Sends a metric or usage event.
  pub async fn send_metric_event<M: Metric>(&self, event: M) {
    self.metrics.send_event(event).await;
  }
}
