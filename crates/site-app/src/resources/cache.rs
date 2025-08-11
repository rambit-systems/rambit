use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Cache, Org};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn cache(
  id: RecordId<Cache>,
) -> Resource<Result<Option<Cache>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_cache, move || id)
}

#[server]
async fn fetch_cache(
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

pub fn caches_in_org(
  org: RecordId<Org>,
) -> Resource<Result<Vec<Cache>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_caches_in_org, move || org)
}

#[server]
async fn fetch_caches_in_org(
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
