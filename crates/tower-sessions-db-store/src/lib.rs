//! A [`SessionStore`] implementer for [`Database`].

use db::Database;
use models::{
  Session,
  dvf::{RecordId, Ulid},
  model::Model,
};
use tower_sessions::{
  SessionStore,
  session::{Id, Record},
  session_store::Error,
};

fn session_id_to_record_id<M: Model>(id: Id) -> RecordId<M> {
  RecordId::from_ulid(Ulid(u128::from_ne_bytes(id.0.to_ne_bytes())))
}

/// A [`SessionStore`] implementer for [`Database`].
#[derive(Clone, Debug)]
pub struct DatabaseStore {
  inner: Database<Session>,
}

impl DatabaseStore {
  /// Construct a new [`SessionStore`].
  pub fn new(db: Database<Session>) -> Self { Self { inner: db } }
}

#[async_trait::async_trait]
impl SessionStore for DatabaseStore {
  async fn save(&self, session_record: &Record) -> Result<(), Error> {
    let session = Session {
      id:     session_id_to_record_id(session_record.id),
      record: session_record.clone(),
    };

    self
      .inner
      .create_model(session)
      .await
      .map(|_| ())
      .map_err(|e| Error::Backend(e.to_string()))
  }

  async fn load(&self, session_id: &Id) -> Result<Option<Record>, Error> {
    self
      .inner
      .fetch_model_by_id(session_id_to_record_id(*session_id))
      .await
      .map(|o| o.map(|s| s.record))
      .map_err(|e| Error::Backend(e.to_string()))
  }

  async fn delete(&self, session_id: &Id) -> Result<(), Error> {
    self
      .inner
      .delete_model(session_id_to_record_id(*session_id))
      .await
      .map(|_| ())
      .map_err(|e| Error::Backend(e.to_string()))
  }
}
