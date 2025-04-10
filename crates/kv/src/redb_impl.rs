use std::{ops::Bound, path::Path};

use hex::health;
use miette::{miette, Context, IntoDiagnostic};
use redb::{ReadableTable, TableDefinition, WriteTransaction};
use tracing::instrument;

use crate::{
  DynTransaction, Key, KvError, KvPrimitive, KvResult, KvTransaction,
  KvTransactional, Value,
};

pub(crate) struct RedbClient(redb::Database);

const TABLE: TableDefinition<Key, Value> = TableDefinition::new("master");

impl RedbClient {
  pub fn new(path: impl AsRef<Path>) -> miette::Result<Self> {
    let path = path.as_ref();
    let path_parent = path.parent();

    match path_parent {
      Some(path_parent) if !path_parent.exists() => {
        tracing::warn!(
          "RedbClient store directory doesn't exist, creating: {}",
          path_parent.display()
        );
        std::fs::create_dir_all(path_parent)
          .into_diagnostic()
          .wrap_err(
            "failed to create directory for non-existent `RedbClient` path",
          )?;
      }
      _ => {}
    }

    Ok(Self(
      redb::Database::create(path)
        .into_diagnostic()
        .context("failed to create redb database")?,
    ))
  }
}

#[async_trait::async_trait]
impl health::HealthReporter for RedbClient {
  fn name(&self) -> &'static str { stringify!(RedbClient) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::IntrensicallyUp.into()
  }
}

/// ReDB transaction.
#[must_use]
pub struct RedbTransaction(
  Option<redb::WriteTransaction>,
  Option<redb::Savepoint>,
);

impl RedbTransaction {
  fn unpack(&mut self) -> KvResult<&mut WriteTransaction> {
    match self {
      Self(Some(ref mut txn), Some(_)) => Ok(txn),
      Self(None, Some(_)) => Err(KvError::PlatformError(miette!(
        "redb transaction already commited"
      ))),
      Self(Some(_), None) => Err(KvError::PlatformError(miette!(
        "redb transaction already rolled back"
      ))),
      _ => Err(KvError::PlatformError(miette!(
        "redb transaction in unexpected state",
      ))),
    }
  }
}

#[async_trait::async_trait]
impl KvTransactional for RedbClient {
  async fn begin_optimistic_transaction(&self) -> KvResult<DynTransaction> {
    tracing::debug!("beginning optimistic transaction");
    let txn = self.0.begin_write()?;
    let savepoint = txn.ephemeral_savepoint()?;
    Ok(DynTransaction::new(RedbTransaction(
      Some(txn),
      Some(savepoint),
    )))
  }
  async fn begin_pessimistic_transaction(&self) -> KvResult<DynTransaction> {
    tracing::debug!("beginning pessimistic transaction");
    let txn = self.0.begin_write()?;
    let savepoint = txn.ephemeral_savepoint()?;
    Ok(DynTransaction::new(RedbTransaction(
      Some(txn),
      Some(savepoint),
    )))
  }
}

#[async_trait::async_trait]
impl KvPrimitive for RedbTransaction {
  #[instrument(skip(self))]
  async fn get(&mut self, key: &Key) -> KvResult<Option<Value>> {
    tracing::debug!("getting key");
    let txn = self.unpack()?;
    let table = txn.open_table(TABLE)?;
    let ag = table.get(key)?;
    Ok(ag.map(|ag| ag.value()))
  }

  #[instrument(skip(self))]
  async fn put(&mut self, key: &Key, value: Value) -> KvResult<()> {
    tracing::debug!("putting key");
    let txn = self.unpack()?;
    let mut table = txn.open_table(TABLE)?;
    table.insert(key, value)?;
    Ok(())
  }

  #[instrument(skip(self))]
  async fn insert(&mut self, key: &Key, value: Value) -> KvResult<()> {
    tracing::debug!("inserting key");
    let txn = self.unpack()?;
    let mut table = txn.open_table(TABLE)?;
    let populated = table.get(key)?.is_some();
    if !populated {
      table.insert(key, value)?;
    }
    Ok(())
  }

  #[instrument(skip(self))]
  async fn scan(
    &mut self,
    start: Bound<Key>,
    end: Bound<Key>,
    limit: Option<u32>,
  ) -> KvResult<Vec<(Key, Value)>> {
    tracing::debug!("scanning keys");
    let txn = self.unpack()?;
    let table = txn.open_table(TABLE)?;
    let range = table.range((start, end))?;
    let mut range =
      range.map(|r| r.map(|(ag_k, ag_v)| (ag_k.value(), ag_v.value())));

    Ok(match limit {
      Some(limit) => range.take(limit as _).try_collect()?,
      None => range.try_collect()?,
    })
  }

  #[instrument(skip(self))]
  async fn delete(&mut self, key: &Key) -> KvResult<bool> {
    tracing::debug!("deleting key");
    let txn = self.unpack()?;
    let mut table = txn.open_table(TABLE)?;
    let deleted_val = table.remove(key)?;
    Ok(deleted_val.is_some())
  }
}

#[async_trait::async_trait]
impl KvTransaction for RedbTransaction {
  async fn commit(&mut self) -> KvResult<()> {
    tracing::debug!("committing transaction");
    if self.0.is_some() && self.1.is_some() {
      let txn = self.0.take().unwrap();
      let savepoint = self.1.take().unwrap();
      txn.commit()?;
      drop(savepoint);
      tracing::debug!("committed transaction");
    } else if self.0.is_none() {
      tracing::error!("transaction already committed");
      return Err(KvError::PlatformError(miette!(
        "redb transaction already commited"
      )));
    } else {
      tracing::error!("transaction already rolled back");
      return Err(KvError::PlatformError(miette!(
        "redb transaction already rolled back"
      )));
    }

    Ok(())
  }

  async fn rollback(&mut self) -> KvResult<()> {
    tracing::debug!("rolling back transaction");
    if self.1.is_some() {
      let savepoint = self.1.take().unwrap();
      drop(savepoint);
      tracing::debug!("transaction rolled back");
    } else {
      tracing::error!("transaction already rolled back");
      return Err(KvError::PlatformError(miette!(
        "redb transaction already rolled back"
      )));
    }

    Ok(())
  }
}
