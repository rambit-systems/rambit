use dvf::RecordId;
use model::Model;
use serde::{Deserialize, Serialize};
use tower_sessions::session::Record;

/// A [tower-sessions](tower_sessions) [`Record`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
  /// The session's ID.
  pub id:     RecordId<Session>,
  /// The session's record data.
  pub record: Record,
}

impl Model for Session {
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];
  const TABLE_NAME: &'static str = "session";
  const UNIQUE_INDICES: &'static [(
    &'static str,
    model::SlugFieldGetter<Self>,
  )] = &[];

  fn id(&self) -> dvf::RecordId<Self> { self.id }
}
