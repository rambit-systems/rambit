use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use models::{dvf::RecordId, model::Model, Entry, Org, PvStore, Store};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn store(
  key: impl Fn() -> RecordId<Store> + Send + Sync + 'static,
) -> Resource<Result<Option<PvStore>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(store_query_scope(), key)
}

pub fn store_query_scope(
) -> QueryScope<RecordId<Store>, Result<Option<PvStore>, ServerFnError>> {
  QueryScope::new(fetch_store).with_invalidation_link(move |s| {
    [Store::TABLE_NAME.to_string(), s.to_string()]
  })
}

#[server(prefix = "/api/sfn")]
async fn fetch_store(
  id: RecordId<Store>,
) -> Result<Option<PvStore>, ServerFnError> {
  use domain::DomainService;

  let domain_service: DomainService = expect_context();

  let store = domain_service
    .meta()
    .fetch_store_by_id(id)
    .await
    .map(|o| o.map(PvStore::from))
    .map_err(|e| {
      tracing::error!("failed to fetch org: {e}");
      ServerFnError::new("internal error")
    })?;

  if let Some(store) = &store {
    authorize_for_org(store.org)?;
  }

  Ok(store)
}

pub fn stores_in_org_query_scope(
) -> QueryScope<RecordId<Org>, Result<Vec<PvStore>, ServerFnError>> {
  QueryScope::new(fetch_stores_in_org)
    .with_invalidation_link(move |_| [Store::TABLE_NAME])
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_stores_in_org(
  org: RecordId<Org>,
) -> Result<Vec<PvStore>, ServerFnError> {
  use domain::DomainService;

  authorize_for_org(org)?;

  let domain_service: DomainService = expect_context();

  let ids = domain_service
    .meta()
    .fetch_stores_by_org(org)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch stores by org: {e}");
      ServerFnError::new("internal error")
    })?;

  let mut models = Vec::with_capacity(ids.len());

  for id in ids {
    models.push(
      domain_service
        .meta()
        .fetch_store_by_id(id)
        .await
        .map_err(|e| {
          tracing::error!("failed to fetch store by id: {e}");
          ServerFnError::new("internal error")
        })?
        .ok_or_else(|| {
          tracing::error!("could not find store just found by org index: {id}");
          ServerFnError::new("internal error")
        })?
        .into(),
    );
  }

  Ok(models)
}

pub fn store_name_is_available_query_scope(
) -> QueryScope<(RecordId<Org>, String), Result<bool, ServerFnError>> {
  QueryScope::new(check_if_store_name_is_available)
    .with_invalidation_link(move |_| [Store::TABLE_NAME])
}

#[server(prefix = "/api/sfn")]
pub async fn check_if_store_name_is_available(
  org_and_name: (RecordId<Org>, String),
) -> Result<bool, ServerFnError> {
  use domain::DomainService;
  use models::dvf::{EntityName, StrictSlug};

  let (org, name) = org_and_name;

  authorize_for_org(org)?;

  let sanitized_name = EntityName::new(StrictSlug::new(name.clone()));
  if name != sanitized_name.clone().to_string() {
    return Err(ServerFnError::new("name is unsanitized"));
  }

  let domain_service: DomainService = expect_context();

  let occupied = domain_service
    .meta()
    .fetch_store_by_org_and_name(org, sanitized_name)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch store by name: {e}");
      ServerFnError::new("internal error")
    })?
    .is_some();

  Ok(!occupied)
}

pub fn entry_count_in_store_query_scope(
) -> QueryScope<RecordId<Store>, Result<u32, ServerFnError>> {
  QueryScope::new(count_entries_in_store).with_invalidation_link(move |s| {
    [
      Entry::TABLE_NAME.to_string(),
      s.to_string(),
      Store::TABLE_NAME.to_string(),
    ]
  })
}

#[server(prefix = "/api/sfn")]
pub async fn count_entries_in_store(
  store: RecordId<Store>,
) -> Result<u32, ServerFnError> {
  use domain::DomainService;

  let domain_service: DomainService = expect_context();
  let store = domain_service
    .meta()
    .fetch_store_by_id(store)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch store: {e}");
      ServerFnError::new("internal error")
    })?
    .ok_or(ServerFnError::new("store does not exist"))?;

  authorize_for_org(store.org)?;

  domain_service
    .meta()
    .count_entries_in_store(store.id)
    .await
    .map_err(|e| {
      tracing::error!("failed to count entries in store ({}): {e}", store.id);
      ServerFnError::new("internal error")
    })
}
