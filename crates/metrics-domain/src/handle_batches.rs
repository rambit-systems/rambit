use std::sync::Arc;

use miette::{Context, IntoDiagnostic};
use reqwest::{Client, Response, header::CONTENT_TYPE};
use serde_json::Value;
use tokio::sync::mpsc;

use super::{MetricsService, MetricsTxConfig};

impl MetricsService {
  /// Long-running task that sends batches of events recieved from the
  /// `Batcher`
  pub(crate) async fn handle_batches(
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
  pub(crate) async fn send_batch(
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
