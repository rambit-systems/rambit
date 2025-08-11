use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Cache, Entry, Org};

#[cfg(feature = "ssr")]
fn authorize_for_org(org: RecordId<Org>) -> Result<(), ServerFnError> {
  use models::AuthUser;

  let auth_user: Option<AuthUser> = use_context();
  let cleared_orgs = auth_user
    .map(|au| au.iter_orgs().collect::<Vec<_>>())
    .unwrap_or_default();
  if !cleared_orgs.contains(&org) {
    return Err(ServerFnError::new("Unauthorized"));
  }
  Ok(())
}

pub fn org(id: RecordId<Org>) -> Resource<Result<Option<Org>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_org, move || id)
}

#[server]
async fn fetch_org(id: RecordId<Org>) -> Result<Option<Org>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(id)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service.fetch_org_by_id(id).await.map_err(|e| {
    tracing::error!("failed to fetch org: {e}");
    ServerFnError::new("internal error")
  })
}

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

pub fn entries_in_org(
  org: RecordId<Org>,
) -> Resource<Result<Vec<Entry>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_entries_in_org, move || org)
}

#[server]
async fn fetch_entries_in_org(
  org: RecordId<Org>,
) -> Result<Vec<Entry>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(org)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service
    .fetch_entries_by_org(org)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch entries by org: {e}");
      ServerFnError::new("internal error")
    })
}
