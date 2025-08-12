use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use models::{dvf::RecordId, Store};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn store(
  key: impl Fn() -> RecordId<Store> + Send + Sync + 'static,
) -> Resource<Result<Option<Store>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(store_query_scope(), key)
}

pub fn store_query_scope(
) -> QueryScope<RecordId<Store>, Result<Option<Store>, ServerFnError>> {
  QueryScope::new(fetch_store)
}

#[server(prefix = "/api/sfn")]
async fn fetch_store(
  id: RecordId<Store>,
) -> Result<Option<Store>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  let prime_domain_service: PrimeDomainService = expect_context();

  let store =
    prime_domain_service
      .fetch_store_by_id(id)
      .await
      .map_err(|e| {
        tracing::error!("failed to fetch org: {e}");
        ServerFnError::new("internal error")
      })?;

  if let Some(store) = &store {
    authorize_for_org(store.org)?;
  }

  Ok(store)
}
