//! A [`SessionStore`] implementer for [`Database`].

use db::{Database, DatabaseError};
use models::{
  RecordId, Session,
  model::{Model, Ulid},
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
  async fn create(&self, session_record: &mut Record) -> Result<(), Error> {
    let session = Session {
      id:     session_id_to_record_id(session_record.id),
      record: session_record.clone().into(),
    };

    self
      .inner
      .insert(&session)
      .await
      .map(|_| ())
      .map_err(|e| Error::Backend(e.to_string()))
  }

  async fn save(&self, session_record: &Record) -> Result<(), Error> {
    let session = Session {
      id:     session_id_to_record_id(session_record.id),
      record: session_record.clone().into(),
    };

    self
      .inner
      .upsert(&session)
      .await
      .map_err(|e| Error::Backend(e.to_string()))?;

    Ok(())
  }

  async fn load(&self, session_id: &Id) -> Result<Option<Record>, Error> {
    Ok(
      self
        .inner
        .get(session_id_to_record_id(*session_id))
        .await
        .map_err(|e| Error::Backend(e.to_string()))?
        .map(|s| Record::try_from(s.record).map_err(Error::Backend))
        .transpose()?,
    )
  }

  async fn delete(&self, session_id: &Id) -> Result<(), Error> {
    match self
      .inner
      .delete(dbg!(session_id_to_record_id(*session_id)))
      .await
    {
      Ok(()) => Ok(()),
      Err(DatabaseError::NotFound(_)) => Ok(()),
      Err(e) => Err(Error::Backend(e.to_string())),
    }
  }
}
