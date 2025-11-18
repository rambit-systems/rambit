//! Types for compute events.

use models::{Entry, Org, RecordId};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

use crate::{Metric, from_unix_timestamp_nanos, to_unix_timestamp_nanos};

/// An compute usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeEvent {
  /// The timestamp of the event. This represents the completion of the
  /// event.
  #[serde(
    serialize_with = "to_unix_timestamp_nanos",
    deserialize_with = "from_unix_timestamp_nanos"
  )]
  pub timestamp:  UtcDateTime,
  /// The ID of the entry being operated on.
  pub entry_id:   RecordId<Entry>,
  /// The nix store path of the entry being operated on.
  pub entry_path: String,
  /// The ID of the org of the entry being operated on.
  pub org_id:     RecordId<Org>,
  /// The number of bytes processed during the compute event.
  pub byte_count: u64,
  /// The type of operation being performed on the entry.
  pub op_type:    OperationType,
}

/// The type of compute operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
  /// An upload operation.
  Upload,
}

impl Metric for ComputeEvent {
  const INDEX_ID: &str = "compute-event";
}

/// A compute usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstampedComputeEvent {
  /// The nix store path of the entry being downloaded.
  pub entry_path: String,
  /// The ID of the org of the entry being downloaded.
  pub org_id:     RecordId<Org>,
  /// The type of operation being performed on the entry.
  pub op_type:    OperationType,
}

impl UnstampedComputeEvent {
  /// Makes a [`ComputeEvent`] out of a [`UnstampedComputeEvent`] with the
  /// remaining information and timestamp.
  pub fn stamp_with_now(
    self,
    entry_id: RecordId<Entry>,
    byte_count: u64,
  ) -> ComputeEvent {
    let timestamp = UtcDateTime::now();
    ComputeEvent {
      timestamp,
      entry_id,
      entry_path: self.entry_path,
      org_id: self.org_id,
      byte_count,
      op_type: self.op_type,
    }
  }
}
