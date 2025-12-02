//! A category-based async batcher.

use std::{
  collections::HashMap,
  hash::Hash,
  time::{Duration, Instant},
};

use tokio::{sync::mpsc, time::interval};

/// Configuration for batch processing behavior.
#[derive(Debug, Clone)]
pub struct BatchConfig {
  /// Maximum number of items in a batch before flushing.
  pub max_size:          usize,
  /// Maximum time to wait before flushing a batch.
  pub max_time:          Duration,
  /// Buffer size for incoming values (defaults to max_size * 10).
  pub value_buffer_size: Option<usize>,
  /// Buffer size for outgoing batches (defaults to 100).
  pub batch_buffer_size: Option<usize>,
}

impl Default for BatchConfig {
  fn default() -> Self {
    Self {
      max_size:          100,
      max_time:          Duration::from_secs(5),
      value_buffer_size: None,
      batch_buffer_size: None,
    }
  }
}

/// Error type for batch operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
  /// The batcher has been closed and is no longer accepting values.
  Closed,
}

impl std::fmt::Display for BatchError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BatchError::Closed => write!(f, "batcher is closed"),
    }
  }
}

impl std::error::Error for BatchError {}

/// A batching mechanism that groups values by category and flushes them
/// based on size or time thresholds.
#[derive(Clone)]
pub struct CategoricalBatcher<G, V> {
  tx: mpsc::Sender<(G, V)>,
}

impl<G: Clone + Eq + Hash + Send + 'static, V: Send + 'static>
  CategoricalBatcher<G, V>
{
  /// Creates a new categorical batcher with the given configuration.
  ///
  /// Returns a tuple of (batcher, receiver) where the batcher is used to add
  /// values and the receiver is used to receive flushed batches.
  pub fn new(config: BatchConfig) -> (Self, mpsc::Receiver<(G, Vec<V>)>) {
    let value_buffer_size =
      config.value_buffer_size.unwrap_or(config.max_size * 10);
    let batch_buffer_size = config.batch_buffer_size.unwrap_or(100);

    let (value_tx, mut value_rx) = mpsc::channel::<(G, V)>(value_buffer_size);
    let (batch_tx, batch_rx) = mpsc::channel(batch_buffer_size);

    tokio::spawn(async move {
      let mut batches: HashMap<G, (Vec<V>, Instant)> = HashMap::new();
      let mut ticker = interval(config.max_time);
      ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
      let mut pending_sends = Vec::new();

      loop {
        tokio::select! {
          maybe_value = value_rx.recv() => {
            match maybe_value {
              Some((group_key, value)) => {
                // Add value to the appropriate batch
                let (values, _) = batches
                  .entry(group_key.clone())
                  .or_insert_with(|| (Vec::new(), Instant::now()));
                values.push(value);

                // Check if batch is full (size-triggered flush)
                if values.len() >= config.max_size
                  && let Some((values, _)) = batches.remove(&group_key) {
                    pending_sends.push((group_key, values));
                  }
              }
              None => {
                // Channel closed, prepare to shutdown
                // Flush all remaining batches
                for (group_key, (values, _)) in batches.drain() {
                  if !values.is_empty() {
                    pending_sends.push((group_key, values));
                  }
                }

                // Send final batches
                for (group_key, values) in pending_sends.drain(..) {
                  // If sending fails, receiver is gone, so we exit
                  if batch_tx.send((group_key, values)).await.is_err() {
                    break;
                  }
                }

                // Exit the task
                return;
              }
            }
          }
          _ = ticker.tick() => {
            // Time-based flush: check all batches for expiration
            let now = Instant::now();
            batches.retain(|group_key, (values, start)| {
              if now.duration_since(*start) >= config.max_time {
                let values = std::mem::take(values);
                if !values.is_empty() {
                  pending_sends.push((group_key.clone(), values));
                }
                false  // Remove from map
              } else {
                true  // Keep in map
              }
            });
          }
        }

        // Send all pending batches outside select! to avoid cancel-safety
        // issues
        for (group_key, values) in pending_sends.drain(..) {
          // If send fails, the receiver has been dropped
          if batch_tx.send((group_key, values)).await.is_err() {
            // Receiver is gone, no point in continuing
            return;
          }
        }
      }
    });

    (Self { tx: value_tx }, batch_rx)
  }

  /// Adds a value to the specified group, waiting if the channel is full.
  ///
  /// Returns an error if the batcher has been shut down.
  pub async fn add(&self, group_key: G, value: V) -> Result<(), BatchError> {
    self
      .tx
      .send((group_key, value))
      .await
      .map_err(|_| BatchError::Closed)
  }
}

#[cfg(test)]
mod tests {
  use tokio::time::timeout;

  use super::*;

  #[tokio::test]
  async fn test_size_based_flush() {
    let config = BatchConfig {
      max_size: 3,
      max_time: Duration::from_secs(10),
      ..Default::default()
    };

    let (batcher, mut receiver) = CategoricalBatcher::new(config);

    // Add 3 values to trigger size-based flush
    batcher.add("group1", 1).await.unwrap();
    batcher.add("group1", 2).await.unwrap();
    batcher.add("group1", 3).await.unwrap();

    let (group, values) = timeout(Duration::from_millis(100), receiver.recv())
      .await
      .unwrap()
      .unwrap();

    assert_eq!(group, "group1");
    assert_eq!(values, vec![1, 2, 3]);
  }

  #[tokio::test]
  async fn test_time_based_flush() {
    let config = BatchConfig {
      max_size: 100,
      max_time: Duration::from_millis(100),
      ..Default::default()
    };

    let (batcher, mut receiver) = CategoricalBatcher::new(config);

    batcher.add("group1", 1).await.unwrap();
    batcher.add("group1", 2).await.unwrap();

    let (group, values) = timeout(Duration::from_millis(200), receiver.recv())
      .await
      .unwrap()
      .unwrap();

    assert_eq!(group, "group1");
    assert_eq!(values, vec![1, 2]);
  }

  #[tokio::test]
  async fn test_multiple_groups() {
    let config = BatchConfig {
      max_size: 2,
      max_time: Duration::from_secs(10),
      ..Default::default()
    };

    let (batcher, mut receiver) = CategoricalBatcher::new(config);

    batcher.add("group1", 1).await.unwrap();
    batcher.add("group2", 10).await.unwrap();
    batcher.add("group1", 2).await.unwrap();
    batcher.add("group2", 20).await.unwrap();

    let mut results = HashMap::new();
    for _ in 0..2 {
      let (group, values) = receiver.recv().await.unwrap();
      results.insert(group, values);
    }

    assert_eq!(results.get("group1"), Some(&vec![1, 2]));
    assert_eq!(results.get("group2"), Some(&vec![10, 20]));
  }

  #[tokio::test]
  async fn test_graceful_shutdown() {
    let config = BatchConfig {
      max_size: 100,
      max_time: Duration::from_secs(10),
      ..Default::default()
    };

    let (batcher, mut receiver) = CategoricalBatcher::new(config);

    batcher.add("group1", 1).await.unwrap();
    batcher.add("group1", 2).await.unwrap();

    // Drop the batcher to trigger shutdown
    drop(batcher);

    // Should still receive the pending batch
    let (group, values) = timeout(Duration::from_millis(100), receiver.recv())
      .await
      .unwrap()
      .unwrap();

    assert_eq!(group, "group1");
    assert_eq!(values, vec![1, 2]);

    // Receiver should now be closed
    assert!(receiver.recv().await.is_none());
  }
}
