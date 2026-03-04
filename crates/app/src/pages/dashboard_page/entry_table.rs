//! Entry table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use domain::db::DatabaseError;
use maud::{Markup, html};
use models::{Abbreviate, Cache, Entry, RecordId};

use crate::{
  components::icons::loading_circle,
  ctx::{Ctx, RequireRequestedOrg, ResponseSeed},
};

const ABBREVIATE_AFTER_COUNT: usize = 5;

/// Renders the full entry table card (shell + initial infill).
pub(super) fn entry_table(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let infill_url = ctx
    .requested_org_url_hook()
    .dashboard_entry_table_infill_url();

  html! {
    div class="flex flex-row items-center gap-2" {
      p class="title" { "Entries" }
      div class="flex-1" {}

      button
        class="btn btn-secondary relative overflow-hidden"
        hx-get=(infill_url)
        hx-target="#entry-table-body"
      {
        "Refresh"
        div class="absolute inset-0 flex flex-row justify-center items-center \
                   btn-secondary htmx-indicator"
        {
          div class="size-4" { (loading_circle()) }
        }
      }
    }

    div class="w-full overflow-x-auto" {
      div class="table w-full" {
        div class="table-header-group" {
          div class="table-row" {
            div class="table-cell" { "Store Path" }
            div class="table-cell" { "Caches" }
            div class="table-cell" { "File Size" }
            div class="table-cell" { "Ref Count" }
          }
        }
        div id="entry-table-body" class="table-row-group min-h-10 relative" {
          (entry_table_data(ctx))
        }
      }
    }
  }
}

/// HTMX infill handler — returns only the row-group contents.
pub(super) async fn entry_table_infill(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  resp.into_stream(entry_table_data(ctx))
}

fn entry_table_data(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let suspense = ctx.suspend(
    move |ctx| async move {
      match fetch_entries(ctx).await {
        Ok(entries) if entries.is_empty() => table_empty_body(4),
        Ok(entries) => entry_rows(entries),
        Err(_) => table_error_body(4),
      }
    },
    table_placeholder_rows(4, 3),
  );

  html! { (suspense) }
}

async fn fetch_entries(
  ctx: Ctx<RequireRequestedOrg>,
) -> Result<Vec<Entry>, DatabaseError> {
  let org_id = ctx.requested_org_url_hook().id();
  let meta = ctx.state().domain.meta();

  let entry_ids = meta.fetch_entries_by_org(org_id).await.inspect_err(|e| {
    tracing::error!("failed to fetch entries by org: {e}");
  })?;

  let mut entries = Vec::with_capacity(entry_ids.len());
  for entry_id in entry_ids {
    if let Some(entry) = meta
      .fetch_entry_by_id(entry_id)
      .await
      .inspect_err(|e| tracing::error!("failed to fetch entry {entry_id}: {e}"))?
    {
      entries.push(entry);
    }
  }

  Ok(entries)
}

fn entry_rows(entries: Vec<Entry>) -> Markup {
  html! {
    @for entry in entries {
      (entry_row(entry))
    }
  }
}

fn entry_row(entry: Entry) -> Markup {
  let abbreviated_path = entry.store_path.abbreviate();
  let full_path = entry.store_path.to_string();

  let cache_count = entry.caches.len();
  let mut caches: Vec<RecordId<Cache>> = entry.caches.clone();
  caches.sort_unstable();
  let visible_caches: Vec<_> =
    caches.into_iter().take(ABBREVIATE_AFTER_COUNT).collect();

  let file_size = entry.intrensic_data.nar_size.to_string();
  let ref_count = entry.intrensic_data.references.len().to_string();

  html! {
    div class="table-row" {
      div class="table-cell font-mono text-sm" {
        span title=(full_path) { (abbreviated_path) }
      }
      div class="table-cell text-sm" {
        @for (i, cache_id) in visible_caches.iter().enumerate() {
          @if i > 0 { ", " }
          span class="font-mono" { (cache_id.to_string()) }
        }
        @if cache_count > ABBREVIATE_AFTER_COUNT { ", …" }
      }
      div class="table-cell" { (file_size) }
      div class="table-cell" { (ref_count) }
    }
  }
}

fn table_empty_body(cols: usize) -> Markup {
  html! {
    div class="table-row" {
      div class="table-cell py-4 text-center text-base-11"
          colspan=(cols.to_string())
      {
        "No entries yet. Upload some from the CLI to see them here."
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
        "Failed to load entries."
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
