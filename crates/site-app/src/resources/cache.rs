use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use models::{dvf::RecordId, model::Model, Cache, Entry, Org, PvCache};

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
  use domain::DomainService;

  let domain_service: DomainService = expect_context();

  let cache = domain_service
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
  use domain::DomainService;

  authorize_for_org(org)?;

  let domain_service: DomainService = expect_context();

  let ids = domain_service.fetch_caches_by_org(org).await.map_err(|e| {
    tracing::error!("failed to fetch caches by org: {e}");
    ServerFnError::new("internal error")
  })?;

  let mut models = Vec::with_capacity(ids.len());

  for id in ids {
    models.push(
      domain_service
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
  use domain::DomainService;
  use models::dvf::{EntityName, StrictSlug};

  authenticate()?;

  let sanitized_name = EntityName::new(StrictSlug::new(name.clone()));
  if name != sanitized_name.clone().to_string() {
    return Err(ServerFnError::new("name is unsanitized"));
  }

  let domain_service: DomainService = expect_context();

  let occupied = domain_service
    .fetch_cache_by_name(sanitized_name)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch cache by name: {e}");
      ServerFnError::new("internal error")
    })?
    .is_some();

  Ok(!occupied)
}

pub fn entry_count_in_cache_query_scope(
) -> QueryScope<RecordId<Cache>, Result<u32, ServerFnError>> {
  QueryScope::new(count_entries_in_cache).with_invalidation_link(move |c| {
    [
      Entry::TABLE_NAME.to_string(),
      c.to_string(),
      Cache::TABLE_NAME.to_string(),
    ]
  })
}

#[server(prefix = "/api/sfn")]
pub async fn count_entries_in_cache(
  cache: RecordId<Cache>,
) -> Result<u32, ServerFnError> {
  use domain::DomainService;

  let domain_service: DomainService = expect_context();
  let cache = domain_service
    .fetch_cache_by_id(cache)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch cache: {e}");
      ServerFnError::new("internal error")
    })?
    .ok_or(ServerFnError::new("cache does not exist"))?;

  authorize_for_org(cache.org)?;

  domain_service
    .count_entries_in_cache(cache.id)
    .await
    .map_err(|e| {
      tracing::error!("failed to count entries in cache ({}): {e}", cache.id);
      ServerFnError::new("internal error")
    })
}
