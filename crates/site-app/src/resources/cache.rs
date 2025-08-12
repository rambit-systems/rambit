use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use models::{dvf::RecordId, Cache, Org};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn cache(
  key: impl Fn() -> RecordId<Cache> + Send + Sync + 'static,
) -> Resource<Result<Option<Cache>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(cache_query_scope(), key)
}

pub fn cache_query_scope(
) -> QueryScope<RecordId<Cache>, Result<Option<Cache>, ServerFnError>> {
  QueryScope::new(fetch_cache)
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_cache(
  id: RecordId<Cache>,
) -> Result<Option<Cache>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  let prime_domain_service: PrimeDomainService = expect_context();

  let cache =
    prime_domain_service
      .fetch_cache_by_id(id)
      .await
      .map_err(|e| {
        tracing::error!("failed to fetch org: {e}");
        ServerFnError::new("internal error")
      })?;

  if let Some(cache) = &cache {
    authorize_for_org(cache.org)?;
  }

  Ok(cache)
}

pub fn caches_in_org_query_scope(
) -> QueryScope<RecordId<Org>, Result<Vec<Cache>, ServerFnError>> {
  QueryScope::new(fetch_caches_in_org)
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_caches_in_org(
  org: RecordId<Org>,
) -> Result<Vec<Cache>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(org)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service
    .fetch_cache_by_org(org)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch caches by org: {e}");
      ServerFnError::new("internal error")
    })
}
