use kv::MockStore;
use model::Model;
use serde::{Deserialize, Serialize};

use super::*;

type TestModelRecordId = model::RecordId<TestModel>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestModel {
  id:   TestModelRecordId,
  name: StrictSlug,
}

impl Model for TestModel {
  const TABLE_NAME: &'static str = "test_model";
  const UNIQUE_INDICES: &'static [(&'static str, fn(&Self) -> EitherSlug)] =
    &[("name", |m| EitherSlug::Strict(m.name.clone()))];
  fn id(&self) -> TestModelRecordId { self.id }
}

#[tokio::test]
async fn test_create_model() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  let created_model = adapter.create_model(model.clone()).await.unwrap();
  assert_eq!(model, created_model);

  let fetched_model = adapter
    .fetch_model_by_id(model.id())
    .await
    .unwrap()
    .unwrap();
  assert_eq!(model, fetched_model);
}

#[tokio::test]
async fn test_fetch_model_by_unique_index() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  adapter.create_model(model.clone()).await.unwrap();

  let fetched_model = adapter
    .fetch_model_by_unique_index(
      "name".to_string(),
      EitherSlug::Strict(model.name.clone()),
    )
    .await
    .unwrap()
    .unwrap();
  assert_eq!(model, fetched_model);
}

#[tokio::test]
async fn test_enumerate_models() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model1 = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test1"),
  };
  let model2 = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test2"),
  };

  adapter.create_model(model1.clone()).await.unwrap();
  adapter.create_model(model2.clone()).await.unwrap();

  let models = adapter.enumerate_models().await.unwrap();
  assert_eq!(models.len(), 2);
  assert!(models.contains(&model1));
  assert!(models.contains(&model2));
}

#[tokio::test]
async fn test_fetch_model_by_id_not_found() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  let fetched_model = adapter.fetch_model_by_id(model.id()).await.unwrap();
  assert!(fetched_model.is_none());
}

#[tokio::test]
async fn test_fetch_model_by_unique_index_not_found() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  adapter.create_model(model.clone()).await.unwrap();

  let fetched_model: Option<TestModel> = adapter
    .fetch_model_by_unique_index(
      "name".to_string(),
      EitherSlug::Strict(StrictSlug::new("not_test")),
    )
    .await
    .unwrap();
  assert!(fetched_model.is_none());
}

#[tokio::test]
async fn test_fetch_model_by_unique_index_does_not_exist() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  adapter.create_model(model.clone()).await.unwrap();

  let result: Result<Option<TestModel>, _> = adapter
    .fetch_model_by_unique_index(
      "not_name".to_string(),
      EitherSlug::Strict(StrictSlug::new("test")),
    )
    .await;
  assert!(matches!(
    result,
    Err(FetchModelByIndexError::IndexDoesNotExistOnModel { .. })
  ));
}

#[tokio::test]
async fn test_fetch_model_by_unique_index_malformed() {
  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  let mock_store = MockStore::new();

  // manually insert the index for a model that doesn't exist
  mock_store.screw_with_internal_data().write().await.insert(
    index_base_key::<TestModel>("name")
      .with_either(EitherSlug::Strict(StrictSlug::new("not_test"))),
    Value::serialize(&model.id()).unwrap(),
  );

  let store = KeyValueStore::from_mock(mock_store);
  let adapter = KvDatabaseAdapter::new(store);

  let result: Result<Option<TestModel>, _> = adapter
    .fetch_model_by_unique_index(
      "name".to_string(),
      EitherSlug::Strict(StrictSlug::new("not_test")),
    )
    .await;
  assert!(matches!(
    result,
    Err(FetchModelByIndexError::IndexMalformed { .. })
  ));
}

#[tokio::test]
async fn test_create_model_already_exists() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  adapter.create_model(model.clone()).await.unwrap();

  let result = adapter.create_model(model.clone()).await;
  assert!(matches!(result, Err(CreateModelError::ModelAlreadyExists)));
}

#[tokio::test]
async fn test_create_model_index_already_exists() {
  let store = KeyValueStore::new_mock();
  let adapter = KvDatabaseAdapter::new(store);

  let model = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };
  let model2 = TestModel {
    id:   model::RecordId::new(),
    name: StrictSlug::new("test"),
  };

  adapter.create_model(model.clone()).await.unwrap();

  let result = adapter.create_model(model2).await;

  assert!(matches!(
    result,
    Err(CreateModelError::IndexAlreadyExists { .. })
  ));
}
