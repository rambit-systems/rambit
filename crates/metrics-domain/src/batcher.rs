use std::{
  collections::HashMap,
  time::{Duration, Instant},
};

use serde_json::Value;
use tokio::{sync::mpsc, time::interval};

pub struct BatchConfig {
  pub max_size: usize,
  pub max_time: Duration,
}

impl Default for BatchConfig {
  fn default() -> Self {
    Self {
      max_size: 100,
      max_time: Duration::from_secs(5),
    }
  }
}

#[derive(Clone)]
pub struct Batcher {
  tx: mpsc::Sender<(&'static str, Value)>,
}

impl Batcher {
  pub fn new(
    config: BatchConfig,
  ) -> (Self, mpsc::Receiver<(&'static str, Vec<Value>)>) {
    let (event_tx, mut event_rx) = mpsc::channel::<(&'static str, Value)>(1000);
    let (batch_tx, batch_rx) = mpsc::channel(100);

    tokio::spawn(async move {
      let mut batches: HashMap<&'static str, (Vec<Value>, Instant)> =
        HashMap::new();
      let mut ticker = interval(config.max_time / 2);
      let mut pending_sends = Vec::new();

      loop {
        tokio::select! {
          Some((idx, event)) = event_rx.recv() => {
            // Add event to the appropriate index batch
            let (events, _) = batches.entry(idx).or_insert_with(|| (Vec::new(), Instant::now()));
            events.push(event);

            // Check if batch is full (size-triggered flush)
            if events.len() >= config.max_size {
              let (events, _) = batches.remove(&idx).unwrap();
              pending_sends.push((idx, events));
            }
          }
          _ = ticker.tick() => {
            // Time-based flush: check all batches for expiration
            batches.retain(|idx, (events, start)| {
              if start.elapsed() >= config.max_time && !events.is_empty() {
                let events = std::mem::take(events);
                pending_sends.push((idx, events));
                false  // Remove from map
              } else {
                true  // Keep in map
              }
            });
          }
        }

        // Send all pending batches outside select! to avoid cancel-safety
        // issues
        for (idx, events) in pending_sends.drain(..) {
          let _ = batch_tx.send((idx, events)).await;
        }
      }
    });

    (Self { tx: event_tx }, batch_rx)
  }

  pub async fn add(&self, index_id: &'static str, event: Value) {
    let _ = self.tx.send((index_id, event)).await;
  }
}
