use std::{
  collections::{hash_map::Entry, HashMap},
  sync::Arc,
};

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

#[allow(clippy::type_complexity)]
#[derive(Debug)]
struct MockDbInner<M> {
  models:    Mutex<HashMap<model::RecordId<M>, M>>,
  u_indices: Mutex<HashMap<String, HashMap<EitherSlug, model::RecordId<M>>>>,
  indices: Mutex<HashMap<String, HashMap<EitherSlug, Vec<model::RecordId<M>>>>>,
}

impl<M> Default for MockDbInner<M> {
  fn default() -> Self {
    Self {
      models:    Mutex::new(HashMap::new()),
      u_indices: Mutex::new(HashMap::new()),
      indices:   Mutex::new(HashMap::new()),
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
    let mut models = self.0.models.lock().await;
    match models.entry(model.id()) {
      Entry::Occupied(_) => return Err(CreateModelError::ModelAlreadyExists),
      Entry::Vacant(vacant_entry) => {
        vacant_entry.insert(model.clone());
      }
    }

    let mut u_indices = self.0.u_indices.lock().await;
    for (u_index_name, u_index_getter) in M::UNIQUE_INDICES.iter() {
      let u_index = u_indices.entry(u_index_name.to_string()).or_default();
      let u_index_value = u_index_getter(&model);

      match u_index.entry(u_index_value.clone()) {
        Entry::Occupied(_) => {
          return Err(CreateModelError::UniqueIndexAlreadyExists {
            index_name:  (*u_index_name).to_owned(),
            index_value: u_index_value,
          })
        }
        Entry::Vacant(vacant_entry) => {
          vacant_entry.insert(model.id());
        }
      }
    }

    let mut indices = self.0.indices.lock().await;
    for (index_name, index_getter) in M::INDICES.iter() {
      let index = indices.entry(index_name.to_string()).or_default();
      let index_value = index_getter(&model);
      let index_ids_for_value = index.entry(index_value).or_default();
      index_ids_for_value.push(model.id());
    }

    Ok(model)
  }

  async fn fetch_model_by_id(
    &self,
    id: model::RecordId<M>,
  ) -> Result<Option<M>, FetchModelError> {
    Ok(self.0.models.lock().await.get(&id).cloned())
  }

  async fn fetch_model_by_unique_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Option<M>, FetchModelByIndexError> {
    if !M::UNIQUE_INDICES.iter().any(|i| i.0 == index_name) {
      return Err(FetchModelByIndexError::IndexDoesNotExistOnModel {
        index_name: index_name.clone(),
      });
    }

    let mut u_indices = self.0.u_indices.lock().await;
    let u_index = u_indices.entry(index_name.clone()).or_default();

    let id = u_index.get(&index_value);
    if let Some(id) = id {
      Ok(self.0.models.lock().await.get(id).cloned())
    } else {
      Ok(None)
    }
  }

  async fn fetch_models_by_index(
    &self,
    index_name: String,
    index_value: EitherSlug,
  ) -> Result<Vec<M>, FetchModelByIndexError> {
    if !M::INDICES.iter().any(|i| i.0 == index_name) {
      return Err(FetchModelByIndexError::IndexDoesNotExistOnModel {
        index_name: index_name.clone(),
      });
    }

    let mut indices = self.0.indices.lock().await;
    let index = indices.entry(index_name.clone()).or_default();

    let ids = index.get(&index_value).cloned().unwrap_or_default();
    let mut models = Vec::with_capacity(ids.len());

    for id in ids {
      models.push(self.fetch_model_by_id(id).await.unwrap().unwrap());
    }

    Ok(models)
  }

  async fn enumerate_models(&self) -> miette::Result<Vec<M>> {
    Ok(self.0.models.lock().await.values().cloned().collect())
  }
}
