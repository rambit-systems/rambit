#![feature(iter_intersperse)]

//! Metrics and usage reporting logic.

mod batcher;

use std::fmt;

pub use metrics;
use metrics::Metric;
use miette::{Context, IntoDiagnostic};
use reqwest::{Client, Url, header::CONTENT_TYPE};
use serde_json::Value;

use self::batcher::{BatchConfig, Batcher};

/// Contains metrics and usage reporting logic.
#[derive(Clone)]
pub struct MetricsService {
  batcher: Batcher,
}

impl fmt::Debug for MetricsService {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(MetricsService)).finish()
  }
}

#[derive(Debug)]
struct MetricsConfig {
  quickwit_url: Url,
}

impl MetricsService {
  /// Creates a new [`MetricsService`].
  pub fn new(quickwit_url: &str) -> miette::Result<Self> {
    let quickwit_url = reqwest::Url::parse(quickwit_url)
      .into_diagnostic()
      .context("failed to parse quickwit url")?;
    let config = MetricsConfig { quickwit_url };
    let client = reqwest::Client::new();
    let (batcher, rx) = Batcher::new(BatchConfig::default());

    tokio::spawn(Self::handle_batches(client, config, rx));

    Ok(Self { batcher })
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
        tracing::error!(err = ?e, "failed to serialize metric event");
        return;
      }
    };

    self.batcher.add(M::INDEX_ID, event).await;
    tracing::info!(index_id = M::INDEX_ID, "receieved metric event");
  }

  async fn handle_batches(
    client: Client,
    config: MetricsConfig,
    mut rx: tokio::sync::mpsc::Receiver<(&'static str, Vec<Value>)>,
  ) {
    while let Some((index_id, event_batch)) = rx.recv().await {
      Self::send_batch(&client, &config, &event_batch, index_id).await;
    }
  }

  #[tracing::instrument(skip(client, events))]
  async fn send_batch(
    client: &Client,
    config: &MetricsConfig,
    events: &[Value],
    index_id: &str,
  ) {
    let url = config
      .quickwit_url
      .join(&format!("/api/v1/{index_id}/ingest"))
      .into_diagnostic()
      .context("failed to parse quickwit url")
      .unwrap();

    let body = events
      .iter()
      .map(serde_json::to_string)
      .filter_map(Result::ok)
      .intersperse("\n".to_owned())
      .collect::<String>();

    match client
      .post(url)
      .header(CONTENT_TYPE, "application/json")
      .body(body)
      .send()
      .await
    {
      Ok(resp) => {
        if let Err(e) = resp.error_for_status_ref() {
          tracing::warn!(err = ?e, "failed to send metric event ingress request");
        } else {
          tracing::info!(
            event_count = events.len(),
            index_id,
            "sent metric event batch"
          );
        }
      }
      Err(e) => {
        tracing::warn!(
          err = ?e,
          "got error response from metric event ingress request"
        );
      }
    };
  }
}
