//! Types for HTTP events.

use std::{net::IpAddr, time::Duration};

use models::model::Ulid;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::UtcDateTime;

use crate::{Metric, from_unix_timestamp_nanos, to_unix_timestamp_nanos};

fn duration_to_micros<S: Serializer>(
  duration: &Duration,
  s: S,
) -> Result<S::Ok, S::Error> {
  s.serialize_u128(duration.as_micros())
}

fn duration_from_micros<'de, D: Deserializer<'de>>(
  d: D,
) -> Result<Duration, D::Error> {
  Ok(Duration::from_micros(
    u128::deserialize(d)?
      .try_into()
      .map_err(serde::de::Error::custom)?,
  ))
}

/// An HTTP event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpEvent {
  /// The timestamp of the event. This represents the completion of the
  /// event.
  #[serde(
    serialize_with = "to_unix_timestamp_nanos",
    deserialize_with = "from_unix_timestamp_nanos"
  )]
  pub timestamp:    UtcDateTime,
  /// The request ID used to correlate events.
  pub request_id:   Ulid,
  /// The service serving the HTTP event.
  pub service_name: String,
  /// The environment that the HTTP event is occurring in.
  pub environment:  String,
  /// The host serving the HTTP event.
  pub host:         String,
  /// The type-specific event details.
  #[serde(flatten)]
  pub details:      EventDetails,
}

/// The details of an [`HttpEvent`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "lowercase")]
pub enum EventDetails {
  /// Details of a request event.
  Request(RequestDetails),
  /// Details of a response event.
  Response(ResponseDetails),
  /// Details of an error event.
  Error(ErrorDetails),
}

/// Details of a request event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDetails {
  /// The HTTP method used in the request.
  pub method:     String,
  /// The path requested.
  pub path:       String,
  /// The URI requested.
  pub uri:        String,
  /// The IP of the client sending the request.
  pub client_ip:  IpAddr,
  /// The user agent used by the client sending the request.
  pub user_agent: String,
}

/// Details of a response event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseDetails {
  /// The status code of the response.
  pub status_code:         u16,
  /// The latency produced in serving the response that errored.
  #[serde(
    rename = "latency_micros",
    serialize_with = "duration_to_micros",
    deserialize_with = "duration_from_micros"
  )]
  pub latency:             Duration,
  /// The size of the response body.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub response_size_bytes: Option<u64>,
}

/// Details of an error event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
  /// The type of error produced.
  pub error_type:    String,
  /// The message included in the produced error.
  pub error_message: String,
  /// The latency produced in serving the response that errored.
  #[serde(
    rename = "latency_micros",
    serialize_with = "duration_to_micros",
    deserialize_with = "duration_from_micros"
  )]
  pub latency:       Duration,
  /// The method of the request whose response errored.
  pub method:        String,
  /// The path of the request whose response errored.
  pub path:          String,
  /// The status code of the error response.
  pub status_code:   u16,
}

impl Metric for HttpEvent {
  const INDEX_ID: &str = "http-event";
}
