use std::{collections::HashMap, str::FromStr};

use model::{Model, RecordId};
use serde::{Deserialize, Serialize};
use tower_sessions::{cookie::time::OffsetDateTime, session::Record};

/// A [tower-sessions](tower_sessions) [`Record`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(table = "session")]
pub struct Session {
  /// The session's ID.
  #[model(id)]
  pub id:     RecordId<Session>,
  /// The session's record data.
  pub record: StoredRecord,
}

/// The stored version of a [`Record`]. Workaround because [`serde_json`] hates
/// 128-bit numbers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredRecord {
  id:          String,
  data:        HashMap<String, serde_json::Value>,
  expiry_date: OffsetDateTime,
}

impl From<Record> for StoredRecord {
  fn from(value: Record) -> Self {
    StoredRecord {
      id:          value.id.0.to_string(),
      data:        value.data,
      expiry_date: value.expiry_date,
    }
  }
}

impl TryFrom<StoredRecord> for Record {
  type Error = String;

  fn try_from(value: StoredRecord) -> Result<Self, Self::Error> {
    Ok(Record {
      id:          tower_sessions::session::Id(
        <i128 as FromStr>::from_str(&value.id).map_err(|e| e.to_string())?,
      ),
      data:        value.data,
      expiry_date: value.expiry_date,
    })
  }
}
