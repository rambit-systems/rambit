use std::{collections::HashMap, fmt, hash::Hash, sync::Arc};

use tokio::{
  sync::RwLock,
  time::{Duration, Instant},
};

/// A cache entry with expiration tracking
#[derive(Clone)]
struct CacheEntry<V> {
  value:      V,
  expires_at: Instant,
}

/// A hashmap-backed cache that expires entries after a configurable duration.
/// Expiration checks and cleanup happen lazily on get operations.
pub struct ExpiringCache<K, V> {
  store: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
  ttl:   Duration,
}

impl<K, V> fmt::Debug for ExpiringCache<K, V> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ExpiringCache")
      .field("store", &())
      .field("ttl", &self.ttl)
      .finish()
  }
}

impl<K, V> ExpiringCache<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  /// Creates a new cache with the specified time-to-live duration
  pub fn new(ttl: Duration) -> Self {
    Self {
      store: Arc::new(RwLock::new(HashMap::new())),
      ttl,
    }
  }

  /// Inserts a key-value pair into the cache
  pub async fn insert(&self, key: K, value: V) {
    let entry = CacheEntry {
      value,
      expires_at: Instant::now() + self.ttl,
    };

    let mut store = self.store.write().await;
    store.insert(key, entry);
  }

  /// Gets a value from the cache if it exists and hasn't expired.
  /// This also triggers cleanup of the expired entry if found.
  pub async fn get(&self, key: &K) -> Option<V> {
    let now = Instant::now();

    // First check with read lock
    {
      let store = self.store.read().await;
      if let Some(entry) = store.get(key)
        && entry.expires_at > now
      {
        return Some(entry.value.clone());
      }
    }

    // If we get here, either the key doesn't exist or it's expired
    // Acquire write lock to clean up expired entry
    let mut store = self.store.write().await;
    if let Some(entry) = store.get(key)
      && entry.expires_at <= now
    {
      store.remove(key);
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use tokio::time::sleep;

  use super::*;

  #[tokio::test]
  async fn test_insert_and_get() {
    let cache = ExpiringCache::new(Duration::from_secs(1));
    cache.insert("key", "value").await;

    assert_eq!(cache.get(&"key").await, Some("value"));
  }

  #[tokio::test]
  async fn test_expiration() {
    let cache = ExpiringCache::new(Duration::from_millis(100));
    cache.insert("key", "value").await;

    // Should exist immediately
    assert_eq!(cache.get(&"key").await, Some("value"));

    // Wait for expiration
    sleep(Duration::from_millis(150)).await;

    // Should be None and cleaned up
    assert_eq!(cache.get(&"key").await, None);
  }
}
