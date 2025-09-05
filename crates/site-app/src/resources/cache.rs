use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use models::{
  dvf::{EntityName, RecordId, StrictSlug},
  model::Model,
  Cache, Org, PvCache,
};

#[cfg(feature = "ssr")]
use crate::resources::{authenticate, authorize_for_org};

pub fn cache(
  key: impl Fn() -> RecordId<Cache> + Send + Sync + 'static,
) -> Resource<Result<Option<PvCache>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(cache_query_scope(), key)
}

pub fn cache_query_scope(
) -> QueryScope<RecordId<Cache>, Result<Option<PvCache>, ServerFnError>> {
  QueryScope::new(fetch_cache).with_invalidation_link(move |c| {
    [Cache::TABLE_NAME.to_string(), c.to_string()]
  })
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_cache(
  id: RecordId<Cache>,
) -> Result<Option<PvCache>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  let prime_domain_service: PrimeDomainService = expect_context();

  let cache = prime_domain_service
    .fetch_cache_by_id(id)
    .await
    .map(|o| o.map(PvCache::from))
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
) -> QueryScope<RecordId<Org>, Result<Vec<PvCache>, ServerFnError>> {
  QueryScope::new(fetch_caches_in_org)
    .with_invalidation_link(move |_| [Cache::TABLE_NAME])
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_caches_in_org(
  org: RecordId<Org>,
) -> Result<Vec<PvCache>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(org)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  let ids = prime_domain_service
    .fetch_caches_by_org(org)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch caches by org: {e}");
      ServerFnError::new("internal error")
    })?;

  let mut models = Vec::with_capacity(ids.len());

  for id in ids {
    models.push(
      prime_domain_service
        .fetch_cache_by_id(id)
        .await
        .map_err(|e| {
          tracing::error!("failed to fetch cache by id: {e}");
          ServerFnError::new("internal error")
        })?
        .ok_or_else(|| {
          tracing::error!("could not find cache just found by org index: {id}");
          ServerFnError::new("internal error")
        })?
        .into(),
    );
  }

  Ok(models)
}

pub fn cache_name_is_available_query_scope(
) -> QueryScope<String, Result<bool, ServerFnError>> {
  QueryScope::new(check_if_cache_name_is_available)
    .with_invalidation_link(move |_| [Cache::TABLE_NAME])
}

#[server(prefix = "/api/sfn")]
pub async fn check_if_cache_name_is_available(
  name: String,
) -> Result<bool, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authenticate()?;

  let prime_domain_service: PrimeDomainService = expect_context();

  let occupied = prime_domain_service
    .fetch_cache_by_name(EntityName::new(StrictSlug::new(name)))
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch cache by name: {e}");
      ServerFnError::new("internal error")
    })?
    .is_some();

  Ok(!occupied)
}
