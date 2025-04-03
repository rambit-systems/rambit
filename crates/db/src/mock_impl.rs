use std::{collections::HashMap, sync::Arc};

use hex::health;
use kv::*;
use model::Model;
use tokio::sync::Mutex;

use crate::{
  CreateModelError, DatabaseAdapter, FetchModelByIndexError, FetchModelError,
};

#[derive(Clone, Debug)]
pub struct MockDatabaseAdapter<M>(Arc<MockDbInner<M>>);

impl<M> Default for MockDatabaseAdapter<M> {
  fn default() -> Self { MockDatabaseAdapter(Arc::new(MockDbInner::default())) }
}

#[derive(Debug)]
struct MockDbInner<M> {
  models:  Mutex<HashMap<model::RecordId<M>, M>>,
  indices: Mutex<HashMap<String, HashMap<EitherSlug, model::RecordId<M>>>>,
}

impl<M> Default for MockDbInner<M> {
  fn default() -> Self {
    Self {
      models:  Mutex::new(HashMap::new()),
      indices: Mutex::new(HashMap::new()),
    }
  }
}

#[async_trait::async_trait]
impl<M: Send + Sync + 'static> health::HealthReporter
  for MockDatabaseAdapter<M>
{
  fn name(&self) -> &'static str { stringify!(MockDatabaseAdapter) }
  async fn health_check(&self) -> health::ComponentHealth {
    health::ComponentHealth::IntrensicallyUp
  }
}

#[async_trait::async_trait]
impl<M: Model> DatabaseAdapter<M> for MockDatabaseAdapter<M> {
  async fn create_model(&self, model: M) -> Result<M, CreateModelError> {
    self.0.models.lock().await.insert(model.id(), model.clone());
    let mut indices = self.0.indices.lock().await;
    for (index_name, index_getter) in M::UNIQUE_INDICES.iter() {
      let index = indices.entry(index_name.to_string()).or_default();
      let index_value = index_getter(&model);
      index.insert(index_value, model.id());
    }

    Ok(model)
  }

  async fn fetch_model_by_id(
    &self,
    id: model::RecordId<M>,
  ) -> Result<Option<M>, FetchModelError> {
    Ok(self.0.models.lock().await.get(&id).cloned())
  }

  async fn fetch_model_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<M>, FetchModelByIndexError> {
    if !M::UNIQUE_INDICES.iter().any(|i| i.0 == index_name) {
      return Err(FetchModelByIndexError::IndexDoesNotExistOnModel {
        index_name: index_name.clone(),
      });
    }

    let mut indices = self.0.indices.lock().await;
    let index = indices.entry(index_name.clone()).or_default();

    let id = index.get(&index_value);
    if let Some(id) = id {
      Ok(self.0.models.lock().await.get(id).cloned())
    } else {
      Ok(None)
    }
  }

  async fn enumerate_models(&self) -> miette::Result<Vec<M>> {
    Ok(self.0.models.lock().await.values().cloned().collect())
  }
}
