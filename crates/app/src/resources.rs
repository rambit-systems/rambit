use domain::db::DatabaseError;
use models::{Cache, Org, PvCache, RecordId};

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

pub async fn fetch_caches_for_requested_org(
  ctx: Ctx<RequireRequestedOrg>,
  _: (),
) -> Result<Vec<PvCache>, DatabaseError> {
  let org_id = ctx.requested_org_url_hook().id();
  let meta = ctx.state().domain.meta();

  let cache_ids = meta.fetch_caches_by_org(org_id).await.inspect_err(|e| {
    tracing::error!("failed to fetch caches by org: {e}");
  })?;

  let mut result = Vec::with_capacity(cache_ids.len());
  for cache_id in cache_ids {
    let Some(cache) =
      meta.fetch_cache_by_id(cache_id).await.inspect_err(|e| {
        tracing::error!("failed to fetch cache {cache_id}: {e}")
      })?
    else {
      continue;
    };
    result.push(cache.into());
  }

  Ok(result)
}

pub async fn fetch_entry_count_for_cache(
  ctx: Ctx<RequireAuth>,
  cache_id: RecordId<Cache>,
) -> Result<Option<AuthResult<u64>>, DatabaseError> {
  let meta = ctx.state().domain.meta();

  // fetch the cache
  let Some(cache) = meta.fetch_cache_by_id(cache_id).await? else {
    return Ok(None);
  };

  // exit early if the user doesn't have access
  if !ctx.auth_user().belongs_to_org(cache.org) {
    return Ok(Some(AuthResult::Unauthorized));
  }

  meta
    .count_entries_in_cache(cache.id)
    .await
    .map(|o| Some(AuthResult::Ok(o)))
}
