use std::ops::Bound;

use slugger::StrictSlug;

use super::*;

#[tokio::test]
async fn test_optimistic_transaction() {
  let store = MockStore::new();

  let mut txn = store.begin_optimistic_transaction().await.unwrap();

  // Test inserting a key-value pair
  let key = Key::new(StrictSlug::new("key1"));
  let value = Value::from("value1");
  txn.put(&key, value.clone()).await.unwrap();

  // Commit the transaction
  txn.commit().await.unwrap();

  // Verify the value is in the store
  let read_value = store.data.read().await.get(&key).cloned();
  assert_eq!(read_value, Some(value));
}

#[tokio::test]
async fn test_pessimistic_transaction() {
  let store = MockStore::new();

  let mut txn = store.begin_pessimistic_transaction().await.unwrap();

  // Test inserting a key-value pair
  let key = Key::new(StrictSlug::new("key2"));
  let value = Value::from("value2");
  txn.put(&key, value.clone()).await.unwrap();

  // Commit the transaction
  txn.commit().await.unwrap();

  // Verify the value is in the store
  let read_value = store.data.read().await.get(&key).cloned();
  assert_eq!(read_value, Some(value));
}

#[tokio::test]
async fn test_conflict_in_optimistic_transaction() {
  let store = MockStore::new();

  // Insert initial data
  store
    .data
    .write()
    .await
    .insert(Key::new(StrictSlug::new("key3")), Value::from("value3"));

  let mut txn = store.begin_optimistic_transaction().await.unwrap();

  // Read the key to add it to the read set
  txn
    .get(&Key::new(StrictSlug::new("key3")))
    .await
    .unwrap()
    .unwrap();

  // Start a new transaction that will conflict
  let mut txn2 = store.begin_optimistic_transaction().await.unwrap();

  // Modify the key that was read in txn
  txn2
    .put(
      &Key::new(StrictSlug::new("key3")),
      Value::from("other_value"),
    )
    .await
    .unwrap();

  txn2.commit().await.unwrap();

  // Commit should fail due to conflict
  let commit_result = txn.commit().await;
  assert!(commit_result.is_err());

  txn.rollback().await.unwrap();
}

#[tokio::test]
async fn test_lock_in_pessimistic_transaction() {
  let store = MockStore::new();

  let mut txn1 = store.begin_pessimistic_transaction().await.unwrap();
  let mut txn2 = store.begin_pessimistic_transaction().await.unwrap();

  let key = Key::new(StrictSlug::new("key4"));

  // Lock the key in txn1
  txn1.put(&key, Value::from("value4")).await.unwrap();

  // Attempt to modify the same key in txn2
  let result = txn2.put(&key, Value::from("other_value")).await;

  // Verify txn2 fails due to lock
  assert!(result.is_err());

  // Commit txn1 and release the lock
  txn1.commit().await.unwrap();

  // Now txn2 should succeed
  let result = txn2.put(&key, Value::from("other_value")).await;
  assert!(result.is_ok());

  txn2.rollback().await.unwrap();
}

#[tokio::test]
async fn test_scan_operation() {
  let store = MockStore::new();

  // Populate store with test data
  let keys_values = vec![
    (Key::new(StrictSlug::new("a")), Value::from("1")),
    (Key::new(StrictSlug::new("b")), Value::from("2")),
    (Key::new(StrictSlug::new("c")), Value::from("3")),
  ];
  {
    let mut data = store.data.write().await;
    for (key, value) in &keys_values {
      data.insert(key.clone(), value.clone());
    }
  }

  let mut txn = store.begin_optimistic_transaction().await.unwrap();

  // Scan a range
  let result = txn
    .scan(
      Bound::Included(Key::new(StrictSlug::new("a"))),
      Bound::Included(Key::new(StrictSlug::new("b"))),
      None,
    )
    .await
    .unwrap();

  assert_eq!(result.len(), 2);

  // switch to hashmap for comparison
  let result_map: HashMap<Key, Value> = result.into_iter().collect();
  let expected_map: HashMap<Key, Value> = keys_values
    .iter()
    .filter(|(key, _)| *(key.to_string()) <= *"b")
    .cloned()
    .collect();
  assert_eq!(result_map, expected_map);

  txn.commit().await.unwrap();
}
