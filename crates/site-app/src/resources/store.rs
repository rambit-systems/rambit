use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use models::{dvf::RecordId, Org, PvStore, Store};

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
  QueryScope::new(fetch_store)
}

#[server(prefix = "/api/sfn")]
async fn fetch_store(
  id: RecordId<Store>,
) -> Result<Option<PvStore>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  let prime_domain_service: PrimeDomainService = expect_context();

  let store = prime_domain_service
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
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_stores_in_org(
  org: RecordId<Org>,
) -> Result<Vec<PvStore>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(org)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service
    .fetch_stores_by_org(org)
    .await
    .map(|v| v.into_iter().map(PvStore::from).collect())
    .map_err(|e| {
      tracing::error!("failed to fetch stores by org: {e}");
      ServerFnError::new("internal error")
    })
}
