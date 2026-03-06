//! Entry table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use maud::{Markup, html};
use models::{Abbreviate, Cache, Entry, RecordId};

use crate::{
  components::{icons::loading_circle, text_data::cache_link},
  ctx::{Ctx, RequireAuth, RequireRequestedOrg, ResponseSeed},
  resources::fetch_entries_for_requested_org,
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

    div class="table" {
      div class="table-header-group" {
        div class="table-row" {
          div class="table-cell" { "Store Path" }
          div class="table-cell" { "Caches" }
          div class="table-cell" { "File Size" }
          div class="table-cell" { "Ref Count" }
        }
      }
      div id="entry-table-body" class="table-row-group min-h-10" {
        (entry_table_data(ctx))
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
      match ctx.fetch_cached(fetch_entries_for_requested_org, ()).await.as_ref() {
        Ok(entries) if entries.is_empty() => table_empty_body(4),
        Ok(entries) => entry_rows(ctx.clone().into(), entries.clone()),
        Err(_) => table_error_body(4),
      }
    },
    table_placeholder_rows(4, 3),
  );

  html! { (suspense) }
}

fn entry_rows(ctx: Ctx<RequireAuth>, entries: Vec<Entry>) -> Markup {
  html! {
    @for entry in entries {
      (entry_row(ctx.clone(), entry))
    }
  }
}

fn entry_row(ctx: Ctx<RequireAuth>, entry: Entry) -> Markup {
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
      div class="table-cell" {
        code title=(full_path) { (abbreviated_path) }
      }
      div class="table-cell" {
        @for (i, cache_id) in visible_caches.iter().enumerate() {
          @if i > 0 { ", " }
          (cache_link(ctx.clone(), *cache_id))
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
