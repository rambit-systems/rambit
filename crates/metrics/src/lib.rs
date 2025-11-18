//! Metric and usage event types.

pub mod egress;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::UtcDateTime;

/// A trait for describing metrics.
pub trait Metric: Serialize {
  /// The index ID of the metric.
  const INDEX_ID: &str;
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
