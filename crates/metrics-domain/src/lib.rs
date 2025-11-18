//! Metrics and usage reporting logic.

use std::{fmt, sync::Arc};

use miette::{Context, IntoDiagnostic};
use models::{Cache, Entry, Org, RecordId, Store};
use reqwest::{Client, Url};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::UtcDateTime;

const EGRESS_EVENT_INDEX_ID: &str = "egress-event";

/// Contains metrics and usage reporting logic.
#[derive(Clone)]
pub struct MetricsService {
  client: Client,
  config: Arc<MetricsConfig>,
}

impl fmt::Debug for MetricsService {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(stringify!(MetricsService)).finish()
  }
}

struct MetricsConfig {
  quickwit_url: Url,
}

impl MetricsService {
  /// Creates a new [`MetricsService`].
  pub fn new(quickwit_url: &str) -> miette::Result<Self> {
    let quickwit_url = reqwest::Url::parse(quickwit_url)
      .into_diagnostic()
      .context("failed to parse quickwit url")?;

    Ok(Self {
      client: Client::new(),
      config: Arc::new(MetricsConfig { quickwit_url }),
    })
  }

  /// Creates a new [`MetricsService`] from environment variables.
  pub fn new_from_env() -> miette::Result<Self> {
    let quickwit_url = std::env::var("QUICKWIT_URL")
      .into_diagnostic()
      .context("failed to read `QUICKWIT_URL`")?;

    Self::new(&quickwit_url)
  }

  #[tracing::instrument(skip(self, payload))]
  async fn send_event<T: Serialize>(
    &self,
    payload: &T,
    index_id: &str,
  ) -> miette::Result<()> {
    let url = self
      .config
      .quickwit_url
      .join(&format!("/api/v1/{index_id}/ingest"))
      .into_diagnostic()
      .context("failed to parse quickwit url")?;

    match self.client.post(url).json(payload).send().await {
      Ok(resp) => {
        if let Err(e) = resp.error_for_status_ref() {
          tracing::warn!(err = ?e, "failed to send metric event ingress request");
        }
      }
      Err(e) => {
        tracing::warn!(
          err = ?e,
          "got error response from metric event ingress request"
        );
      }
    };

    Ok(())
  }
}

fn to_unix_timestamp_nanos<S: Serializer>(
  datetime: &UtcDateTime,
  s: S,
) -> Result<S::Ok, S::Error> {
  s.serialize_i128(datetime.unix_timestamp_nanos())
}

fn from_unix_timestamp_nanos<'de, D: Deserializer<'de>>(
  d: D,
) -> Result<UtcDateTime, D::Error> {
  UtcDateTime::from_unix_timestamp_nanos(i128::deserialize(d)?)
    .map_err(serde::de::Error::custom)
}

/// An egress usage event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgressEvent {
  /// The timestamp of the event. This represents the completion of the event.
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

/// An egress usage event without a timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelessEgressEvent {
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

impl TimelessEgressEvent {
  /// Makes an [`EgressEvent`] out of a [`TimelessEgressEvent`] with the current
  /// time.
  pub fn stamp_with_now(self) -> EgressEvent {
    EgressEvent {
      timestamp:  UtcDateTime::now(),
      entry_id:   self.entry_id,
      entry_path: self.entry_path,
      cache_id:   self.cache_id,
      store_id:   self.store_id,
      org_id:     self.org_id,
      byte_count: self.byte_count,
    }
  }
}

impl MetricsService {
  /// Sends an egress usage event to the aggregator.
  pub async fn send_egress_event(
    &self,
    event: &EgressEvent,
  ) -> miette::Result<()> {
    self.send_event(event, EGRESS_EVENT_INDEX_ID).await
  }
}
