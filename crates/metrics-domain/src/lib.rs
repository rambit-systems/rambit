#![feature(iter_intersperse)]

//! Metrics and usage reporting logic.

mod handle_batches;

use std::{fmt, sync::Arc};

use category_batcher::{BatchConfig, CategoricalBatcher};
pub use metrics;
use metrics::Metric;
use miette::{Context, IntoDiagnostic};
use reqwest::{Client, Url};
use serde_json::Value;

/// Contains metrics and usage reporting logic.
#[derive(Clone)]
pub struct MetricsService {
  batcher: Arc<CategoricalBatcher<&'static str, Value>>,
  _client: Client,
}

impl fmt::Debug for MetricsService {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(MetricsService)).finish()
  }
}

#[derive(Debug)]
struct MetricsTxConfig {
  quickwit_url: Url,
}

impl MetricsService {
  /// Creates a new [`MetricsService`].
  pub fn new(quickwit_url: &str) -> miette::Result<Self> {
    let quickwit_url = reqwest::Url::parse(quickwit_url)
      .into_diagnostic()
      .context("failed to parse quickwit url")?;
    let config = Arc::new(MetricsTxConfig { quickwit_url });
    let client = reqwest::Client::new();
    let (batcher, rx) = CategoricalBatcher::new(BatchConfig::default());

    tokio::spawn(Self::handle_batches(client.clone(), config, rx));

    Ok(Self {
      batcher: Arc::new(batcher),
      _client: client,
    })
  }

  /// Creates a new [`MetricsService`] from environment variables.
  pub fn new_from_env() -> miette::Result<Self> {
    let quickwit_url = std::env::var("QUICKWIT_URL")
      .into_diagnostic()
      .context("failed to read `QUICKWIT_URL`")?;

    Self::new(&quickwit_url)
  }

  /// Sends an event to the metrics service.
  pub async fn send_event<M: Metric>(&self, event: M) {
    let event = match serde_json::to_value(&event) {
      Ok(event) => event,
      Err(e) => {
        tracing::error!(
          index_id = M::INDEX_ID,
          err = ?e,
          "failed to serialize metric event"
        );
        return;
      }
    };

    let _ = self.batcher.add(M::INDEX_ID, event).await.inspect_err(|e| {
      tracing::error!(
        index_id = M::INDEX_ID,
        err = ?e,
        "failed to send metric event to batcher"
      );
    });
    tracing::info!(index_id = M::INDEX_ID, "receieved metric event");
  }
}
