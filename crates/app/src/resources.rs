use domain::db::DatabaseError;
use models::{Cache, Entry, Org, PvCache, PvStore, RecordId, Store};

use crate::ctx::{Ctx, RequireAuth, RequireRequestedOrg};

#[derive(Clone)]
pub enum AuthResult<T> {
  Ok(T),
  Unauthorized,
}

pub async fn fetch_org(
  ctx: Ctx<RequireAuth>,
  id: RecordId<Org>,
) -> Result<Option<Org>, DatabaseError> {
  ctx.state().domain.meta().fetch_org_by_id(id).await
}

pub async fn fetch_cache(
  ctx: Ctx<RequireAuth>,
  id: RecordId<Cache>,
) -> Result<Option<Cache>, DatabaseError> {
  ctx.state().domain.meta().fetch_cache_by_id(id).await
}

pub async fn fetch_store(
  ctx: Ctx<RequireAuth>,
  id: RecordId<Store>,
) -> Result<Option<Store>, DatabaseError> {
  ctx.state().domain.meta().fetch_store_by_id(id).await
}

pub async fn fetch_entry(
  ctx: Ctx<RequireAuth>,
  id: RecordId<Entry>,
) -> Result<Option<Entry>, DatabaseError> {
  ctx.state().domain.meta().fetch_entry_by_id(id).await
}

pub async fn fetch_caches_for_requested_org(
  ctx: Ctx<RequireRequestedOrg>,
  _: (),
) -> Result<Vec<PvCache>, DatabaseError> {
  let org_id = ctx.requested_org_url_hook().id();
  let auth_ctx = ctx.clone().into_require_auth();

  let cache_ids = ctx
    .state()
    .domain
    .meta()
    .fetch_caches_by_org(org_id)
    .await
    .inspect_err(|e| tracing::error!("failed to fetch caches by org: {e}"))?;

  let mut result = Vec::with_capacity(cache_ids.len());
  for id in cache_ids {
    match auth_ctx.fetch_cached(fetch_cache, id).await.as_ref() {
      Ok(Some(cache)) => result.push(cache.clone().into()),
      Ok(None) => {}
      Err(e) => tracing::error!("failed to fetch cache {id}: {e}"),
    }
  }

  Ok(result)
}

pub async fn fetch_stores_for_requested_org(
  ctx: Ctx<RequireRequestedOrg>,
  _: (),
) -> Result<Vec<PvStore>, DatabaseError> {
  let org_id = ctx.requested_org_url_hook().id();
  let auth_ctx = ctx.clone().into_require_auth();

  let store_ids = ctx
    .state()
    .domain
    .meta()
    .fetch_stores_by_org(org_id)
    .await
    .inspect_err(|e| tracing::error!("failed to fetch stores by org: {e}"))?;

  let mut result = Vec::with_capacity(store_ids.len());
  for id in store_ids {
    match auth_ctx.fetch_cached(fetch_store, id).await.as_ref() {
      Ok(Some(store)) => result.push(store.clone().into()),
      Ok(None) => {}
      Err(e) => tracing::error!("failed to fetch store {id}: {e}"),
    }
  }

  Ok(result)
}

pub async fn fetch_entries_for_requested_org(
  ctx: Ctx<RequireRequestedOrg>,
  _: (),
) -> Result<Vec<Entry>, DatabaseError> {
  let org_id = ctx.requested_org_url_hook().id();
  let auth_ctx = ctx.clone().into_require_auth();

  let entry_ids = ctx
    .state()
    .domain
    .meta()
    .fetch_entries_by_org(org_id)
    .await
    .inspect_err(|e| tracing::error!("failed to fetch entries by org: {e}"))?;

  let mut result = Vec::with_capacity(entry_ids.len());
  for id in entry_ids {
    match auth_ctx.fetch_cached(fetch_entry, id).await.as_ref() {
      Ok(Some(entry)) => result.push(entry.clone()),
      Ok(None) => {}
      Err(e) => tracing::error!("failed to fetch entry {id}: {e}"),
    }
  }

  Ok(result)
}

pub async fn fetch_entry_count_for_cache(
  ctx: Ctx<RequireAuth>,
  cache_id: RecordId<Cache>,
) -> Result<Option<AuthResult<u64>>, DatabaseError> {
  let meta = ctx.state().domain.meta();

  let Some(cache) = meta.fetch_cache_by_id(cache_id).await? else {
    return Ok(None);
  };

  if !ctx.auth_user().belongs_to_org(cache.org) {
    return Ok(Some(AuthResult::Unauthorized));
  }

  meta
    .count_entries_in_cache(cache.id)
    .await
    .map(|o| Some(AuthResult::Ok(o)))
}

pub async fn fetch_entry_count_for_store(
  ctx: Ctx<RequireAuth>,
  store_id: RecordId<Store>,
) -> Result<Option<AuthResult<u64>>, DatabaseError> {
  let meta = ctx.state().domain.meta();

  let Some(store) = meta.fetch_store_by_id(store_id).await? else {
    return Ok(None);
  };

  if !ctx.auth_user().belongs_to_org(store.org) {
    return Ok(Some(AuthResult::Unauthorized));
  }

  meta
    .count_entries_in_store(store.id)
    .await
    .map(|o| Some(AuthResult::Ok(o)))
}
