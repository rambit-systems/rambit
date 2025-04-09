//! A mock implementation of [`KvTransactional`]. Follows TiKV semantics around
//! transactions.

#[cfg(test)]
mod tests;

use std::{
  collections::{HashMap, HashSet},
  ops::Bound,
  sync::Arc,
};

use hex::health;
use tokio::sync::{Mutex, RwLock};

use crate::{
  key::Key, value::Value, DynTransaction, KvError, KvPrimitive, KvResult,
  KvTransaction, KvTransactional,
};

/// A mock key-value store.
#[derive(Clone)]
pub struct MockStore {
  data:  Arc<RwLock<HashMap<Key, Value>>>,
  locks: Arc<Mutex<HashSet<Key>>>, // Set of keys currently locked
}

impl MockStore {
  /// Create a new mock store.
  pub fn new() -> Arc<Self> {
    Arc::new(Self {
      data:  Arc::new(RwLock::new(HashMap::new())),
      locks: Arc::new(Mutex::new(HashSet::new())),
    })
  }

  /// Screw with the internal data of the store. This is useful for testing.
  #[allow(dead_code)]
  pub fn screw_with_internal_data(&self) -> &RwLock<HashMap<Key, Value>> {
    &self.data
  }
}

#[async_trait::async_trait]
impl KvTransactional for MockStore {
  async fn begin_optimistic_transaction(&self) -> KvResult<DynTransaction> {
    Ok(DynTransaction::new(OptimisticTransaction {
      store:     self.clone(),
      read_set:  HashMap::new(),
      write_set: HashMap::new(),
    }))
  }

  async fn begin_pessimistic_transaction(&self) -> KvResult<DynTransaction> {
    Ok(DynTransaction::new(PessimisticTransaction {
      store:       self.clone(),
      locked_keys: HashSet::new(),
      write_set:   HashMap::new(),
    }))
  }
}

#[health::async_trait]
impl health::HealthReporter for MockStore {
  fn name(&self) -> &'static str { stringify!(MockStore) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::IntrensicallyUp.into()
  }
}

/// A transaction error.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TransactionError {
  /// The key was locked.
  #[error("key locked: `{0}`")]
  KeyLocked(Key),
  /// There was a key conflict.
  #[error("key conflict: `{0}`")]
  KeyConflict(Key),
  /// Some other error occurred.
  #[error("other: {0:?}")]
  Other(String),
}

impl From<TransactionError> for KvError {
  fn from(error: TransactionError) -> Self {
    KvError::PlatformError(error.into())
  }
}

/// An optimistic transaction.
pub struct OptimisticTransaction {
  store:     MockStore,
  read_set:  HashMap<Key, Option<Value>>,
  write_set: HashMap<Key, Value>,
}

impl Drop for OptimisticTransaction {
  fn drop(&mut self) {
    if !self.read_set.is_empty() || !self.write_set.is_empty() {
      panic!(
        "Optimistic transaction dropped without commit or rollback. Read Set: \
         {:?}, Write Set: {:?}",
        self.read_set, self.write_set
      );
    }
  }
}

impl OptimisticTransaction {
  async fn check_conflicts(&self) -> Result<(), TransactionError> {
    for (key, value) in &self.read_set {
      if value.as_ref() != self.store.data.read().await.get(key) {
        return Err(TransactionError::KeyConflict(key.clone()));
      }
    }
    Ok(())
  }
}

#[async_trait::async_trait]
impl KvPrimitive for OptimisticTransaction {
  async fn get(&mut self, key: &Key) -> KvResult<Option<Value>> {
    let data = self.store.data.read().await;
    let value = data.get(key).cloned();
    self.read_set.insert(key.clone(), value.clone());
    Ok(value)
  }
  async fn put(&mut self, key: &Key, value: Value) -> KvResult<()> {
    self
      .read_set
      .insert(key.clone(), self.store.data.read().await.get(key).cloned());
    self.write_set.insert(key.clone(), value);
    Ok(())
  }
  async fn insert(&mut self, key: &Key, value: Value) -> KvResult<()> {
    let data = self.store.data.write().await;
    if data.contains_key(key) {
      return Err(KvError::PlatformError(miette::miette!(
        "Key already exists"
      )));
    }
    self.read_set.insert(key.clone(), data.get(key).cloned());
    self.write_set.insert(key.clone(), value.clone());
    Ok(())
  }
  async fn scan(
    &mut self,
    start: Bound<Key>,
    end: Bound<Key>,
    limit: Option<u32>,
  ) -> KvResult<Vec<(Key, Value)>> {
    let data = self.store.data.read().await;
    let mut result = Vec::new();
    for (key, value) in data.iter() {
      if match &start {
        Bound::Included(start) => key >= start,
        Bound::Excluded(start) => key > start,
        Bound::Unbounded => true,
      } && match &end {
        Bound::Included(end) => key <= end,
        Bound::Excluded(end) => key < end,
        Bound::Unbounded => true,
      } {
        self.read_set.insert(key.clone(), Some(value.clone()));
        result.push((key.clone(), value.clone()));
        if let Some(limit) = limit {
          if result.len() == limit as usize {
            break;
          }
        }
      }
    }
    Ok(result)
  }
  async fn delete(&mut self, key: &Key) -> KvResult<bool> {
    self
      .read_set
      .insert(key.clone(), self.store.data.read().await.get(key).cloned());
    self.write_set.insert(key.clone(), Value::new(vec![]));
    Ok(true)
  }
}

#[async_trait::async_trait]
impl KvTransaction for OptimisticTransaction {
  async fn commit(&mut self) -> KvResult<()> {
    self.check_conflicts().await?;
    let mut data = self.store.data.write().await;
    for (key, value) in self.write_set.drain() {
      data.insert(key, value);
    }
    self.read_set.clear();
    Ok(())
  }

  async fn rollback(&mut self) -> KvResult<()> {
    self.read_set.clear();
    self.write_set.clear();
    Ok(())
  }
}

/// A pessimistic transaction.
pub struct PessimisticTransaction {
  store:       MockStore,
  locked_keys: HashSet<Key>,
  write_set:   HashMap<Key, Value>,
}

impl Drop for PessimisticTransaction {
  fn drop(&mut self) {
    if !self.locked_keys.is_empty() || !self.write_set.is_empty() {
      panic!(
        "Pessimistic transaction dropped without commit or rollback. Keys: \
         {:?}, Write Set: {:?}",
        self.locked_keys, self.write_set
      );
    }
  }
}

impl PessimisticTransaction {
  async fn lock_key(&mut self, key: &Key) -> Result<(), TransactionError> {
    if self.locked_keys.contains(key) {
      return Ok(());
    }
    let mut locks = self.store.locks.lock().await;
    if locks.contains(key) {
      return Err(TransactionError::KeyLocked(key.clone()));
    }
    locks.insert(key.clone());
    self.locked_keys.insert(key.clone());
    Ok(())
  }

  async fn unlock_keys(&self) {
    let mut locks = self.store.locks.lock().await;
    for key in &self.locked_keys {
      locks.remove(key);
    }
  }
}

#[async_trait::async_trait]
impl KvPrimitive for PessimisticTransaction {
  async fn get(&mut self, key: &Key) -> KvResult<Option<Value>> {
    self.lock_key(key).await?;
    let data = self.store.data.read().await;
    Ok(data.get(key).cloned())
  }
  async fn put(&mut self, key: &Key, value: Value) -> KvResult<()> {
    self.lock_key(key).await?;
    self.write_set.insert(key.clone(), value);
    Ok(())
  }
  async fn insert(&mut self, key: &Key, value: Value) -> KvResult<()> {
    self.lock_key(key).await?;
    if self.store.data.read().await.contains_key(key) {
      return Err(KvError::PlatformError(miette::miette!(
        "Key already exists"
      )));
    }
    self.write_set.insert(key.clone(), value);
    Ok(())
  }
  async fn scan(
    &mut self,
    start: Bound<Key>,
    end: Bound<Key>,
    limit: Option<u32>,
  ) -> KvResult<Vec<(Key, Value)>> {
    let data = self.store.data.read().await;
    let mut result = Vec::new();
    for (key, value) in data.iter() {
      if match &start {
        Bound::Included(start) => key >= start,
        Bound::Excluded(start) => key > start,
        Bound::Unbounded => true,
      } && match &end {
        Bound::Included(end) => key <= end,
        Bound::Excluded(end) => key < end,
        Bound::Unbounded => true,
      } {
        result.push((key.clone(), value.clone()));
        if let Some(limit) = limit {
          if result.len() == limit as usize {
            break;
          }
        }
      }
    }
    Ok(result)
  }
  async fn delete(&mut self, key: &Key) -> KvResult<bool> {
    self.lock_key(key).await?;
    self.write_set.insert(key.clone(), Value::new(vec![]));
    Ok(true)
  }
}

#[async_trait::async_trait]
impl KvTransaction for PessimisticTransaction {
  async fn commit(&mut self) -> KvResult<()> {
    let mut data = self.store.data.write().await;
    for (key, value) in self.write_set.drain() {
      data.insert(key, value);
    }
    self.unlock_keys().await;
    self.locked_keys.clear();
    self.write_set.clear();
    Ok(())
  }

  async fn rollback(&mut self) -> KvResult<()> {
    self.unlock_keys().await;
    self.locked_keys.clear();
    self.write_set.clear();
    Ok(())
  }
}
