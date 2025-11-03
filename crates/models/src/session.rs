use model::{Model, RecordId};
use serde::{Deserialize, Serialize};
use tower_sessions::session::Record;

/// A [tower-sessions](tower_sessions) [`Record`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(table = "session")]
pub struct Session {
  /// The session's ID.
  #[model(id)]
  pub id:     RecordId<Session>,
  /// The session's record data.
  pub record: Record,
}
