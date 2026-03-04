//! Cache table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use domain::db::DatabaseError;
use maud::{Markup, html};
use models::{PvCache, Visibility};

use crate::{
  components::icons::*,
  ctx::{Ctx, RequireRequestedOrg, ResponseSeed},
};

/// Renders the full cache table card (shell + initial infill).
pub(super) fn cache_table(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let org_hook = ctx.requested_org_url_hook();
  let infill_url = org_hook.dashboard_cache_table_infill_url();
  let create_url = org_hook.create_cache_url();

  html! {
    div class="flex flex-row items-center gap-2" {
      p class="title" { "Caches" }
      div class="flex-1" {}

      button
        class="btn btn-secondary relative overflow-hidden"
        hx-get=(infill_url)
        hx-target="#cache-table-body"
      {
        "Refresh"
        div class="absolute inset-0 flex flex-row justify-center items-center \
                   btn-secondary htmx-indicator"
        {
          div class="size-4" { (loading_circle()) }
        }
      }

      a href=(create_url) class="btn btn-primary-subtle" { "Create..." }
    }

    div class="w-full overflow-x-auto" {
      div class="table w-full" {
        div class="table-header-group" {
          div class="table-row" {
            div class="table-cell" { "Name" }
            div class="table-cell" { "Visibility" }
            div class="table-cell" { "Entry Count" }
          }
        }
        div id="cache-table-body" class="table-row-group" {
          (cache_table_data(ctx))
        }
      }
    }
  }
}

/// HTMX infill handler — returns only the row-group contents.
pub(super) async fn cache_table_infill(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  resp.into_stream(cache_table_data(ctx))
}

fn cache_table_data(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let suspense = ctx.suspend(
    move |ctx| async move {
      match fetch_caches(ctx).await {
        Ok(caches) if caches.is_empty() => table_empty_body(3),
        Ok(caches) => cache_rows(caches),
        Err(_) => table_error_body(3),
      }
    },
    table_placeholder_rows(3, 3),
  );

  html! { (suspense) }
}

async fn fetch_caches(
  ctx: Ctx<RequireRequestedOrg>,
) -> Result<Vec<(PvCache, u64)>, DatabaseError> {
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
    let count = meta
      .count_entries_in_cache(cache_id)
      .await
      .inspect_err(|e| {
        tracing::error!("failed to count entries in cache {cache_id}: {e}");
      })
      .unwrap_or(0);
    result.push((cache.into(), count));
  }

  Ok(result)
}

fn cache_rows(caches: Vec<(PvCache, u64)>) -> Markup {
  html! {
    @for (cache, count) in caches {
      (cache_row(cache, count))
    }
  }
}

fn cache_row(cache: PvCache, entry_count: u64) -> Markup {
  html! {
    div class="table-row" {
      div class="table-cell font-mono" { (cache.name.as_ref()) }
      div class="table-cell" {
        div class="flex flex-row items-center gap-1" {
          (cache.visibility.to_string())
          @if matches!(cache.visibility, Visibility::Private) {
            div class="size-4 stroke-base-11/75 stroke-[2.0]" {
              (lock_closed_hero_icon())
            }
          }
        }
      }
      div class="table-cell" { (entry_count.to_string()) }
    }
  }
}

fn table_empty_body(cols: usize) -> Markup {
  html! {
    div class="table-row" {
      div class=(format!("table-cell py-4 text-center text-base-11"))
          colspan=(cols.to_string())
      {
        "No caches yet."
      }
    }
  }
}

fn table_error_body(cols: usize) -> Markup {
  html! {
    div class="table-row" {
      div class="table-cell py-4 text-center text-critical-11"
          colspan=(cols.to_string())
      {
        "Failed to load caches."
      }
    }
  }
}

fn table_placeholder_rows(cols: usize, n: usize) -> Markup {
  html! {
    @for _ in 0..n {
      div class="table-row" {
        div class="table-cell py-2" colspan=(cols.to_string()) {
          div class="h-4 rounded bg-base-4 animate-pulse" {}
        }
      }
    }
  }
}
