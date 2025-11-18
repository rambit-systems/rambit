//! Types for egress events.

use models::{Cache, Entry, Org, RecordId, Store};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

use crate::{Metric, from_unix_timestamp_nanos, to_unix_timestamp_nanos};

/// An egress usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgressEvent {
  /// The timestamp of the event. This represents the completion of the
  /// event.
  #[serde(
    serialize_with = "to_unix_timestamp_nanos",
    deserialize_with = "from_unix_timestamp_nanos"
  )]
  pub timestamp:  UtcDateTime,
  /// The ID of the entry being downloaded.
  pub entry_id:   RecordId<Entry>,
  /// The nix store path of the entry being downloaded.
  pub entry_path: String,
  /// The ID of the cache of the entry being downloaded.
  pub cache_id:   RecordId<Cache>,
  /// The ID of the store of the entry being downloaded.
  pub store_id:   RecordId<Store>,
  /// The ID of the org of the entry being downloaded.
  pub org_id:     RecordId<Org>,
  /// The number of bytes served during the egress event.
  pub byte_count: u64,
}

impl Metric for EgressEvent {
  const INDEX_ID: &str = "egress-event";
}

/// An egress usage event prepared beforehand.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstampedEgressEvent {
  /// The ID of the entry being downloaded.
  pub entry_id:   RecordId<Entry>,
  /// The nix store path of the entry being downloaded.
  pub entry_path: String,
  /// The ID of the cache of the entry being downloaded.
  pub cache_id:   RecordId<Cache>,
  /// The ID of the store of the entry being downloaded.
  pub store_id:   RecordId<Store>,
  /// The ID of the org of the entry being downloaded.
  pub org_id:     RecordId<Org>,
}

impl UnstampedEgressEvent {
  /// Makes an [`EgressEvent`] out of a [`UnstampedEgressEvent`] with the
  /// remaining information and timestamp.
  pub fn stamp_with_now(self, byte_count: u64) -> EgressEvent {
    let timestamp = UtcDateTime::now();
    EgressEvent {
      timestamp,
      entry_id: self.entry_id,
      entry_path: self.entry_path,
      cache_id: self.cache_id,
      store_id: self.store_id,
      org_id: self.org_id,
      byte_count,
    }
  }
}
