#![feature(iter_intersperse)]

//! Metrics and usage reporting logic.

mod batcher;

use std::{fmt, sync::Arc};

pub use metrics;
use metrics::Metric;
use miette::{Context, IntoDiagnostic};
use reqwest::{Client, Response, Url, header::CONTENT_TYPE};
use serde_json::Value;
use tokio::sync::mpsc;

use self::batcher::{BatchConfig, Batcher};

/// Contains metrics and usage reporting logic.
#[derive(Clone)]
pub struct MetricsService {
  batcher: Arc<Batcher>,
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
    let (batcher, rx) = Batcher::new(BatchConfig::default());

    tokio::spawn(Self::handle_batches(client, config, rx));

    Ok(Self {
      batcher: Arc::new(batcher),
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
        tracing::error!(err = ?e, "failed to serialize metric event");
        return;
      }
    };

    self.batcher.add(M::INDEX_ID, event).await;
    tracing::info!(index_id = M::INDEX_ID, "receieved metric event");
  }

  /// Long-running task that sends batches of events recieved from the `Batcher`
  async fn handle_batches(
    client: Client,
    config: Arc<MetricsTxConfig>,
    mut rx: mpsc::Receiver<(&'static str, Vec<Value>)>,
  ) {
    while let Some((index_id, event_batch)) = rx.recv().await {
      tokio::spawn(Self::send_batch(
        client.clone(),
        config.clone(),
        event_batch.into_boxed_slice(),
        index_id,
      ));
    }
  }

  /// Sends a batch of events to Quickwit
  #[tracing::instrument(skip(client, events))]
  async fn send_batch(
    client: Client,
    config: Arc<MetricsTxConfig>,
    events: Box<[Value]>,
    index_id: &'static str,
  ) {
    let url = config
      .quickwit_url
      .join(&format!("/api/v1/{index_id}/ingest"))
      .into_diagnostic()
      .context("failed to parse quickwit url")
      .unwrap();

    // newline-delimited JSON records
    let body = events
      .iter()
      .map(serde_json::to_string)
      .filter_map(Result::ok)
      .intersperse("\n".to_owned())
      .collect::<String>();

    // send the events
    let result = client
      .post(url)
      .header(CONTENT_TYPE, "application/json")
      .body(body)
      .send()
      .await;

    // don't panic or propagate error, only log it
    match result.map(Response::error_for_status) {
      Ok(Err(e)) => {
        tracing::warn!(err = ?e, "failed to send metric event ingress request");
      }
      Ok(Ok(_)) => {
        tracing::info!(
          event_count = events.len(),
          index_id,
          "sent metric event batch"
        );
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
