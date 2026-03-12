//! Entry table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use maud::{Markup, html};
use models::{Abbreviate, Cache, Entry, RecordId};

use crate::{
  components::{
    icons::loading_circle,
    table::{
      overlaid_critical_message_table_body, overlaid_loading_table_body,
      overlaid_message_table_body,
    },
    text_data::cache_link,
  },
  ctx::{Ctx, RequireAuth, RequireRequestedOrg, ResponseSeed},
  hooks::OrgUrlHook,
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
      match ctx
        .fetch_cached(fetch_entries_for_requested_org, ())
        .await
        .as_ref()
      {
        Ok(entries) if entries.is_empty() => empty_body(),
        Ok(entries) => entry_rows(ctx.clone().into(), entries.clone()),
        Err(_) => error_body(),
      }
    },
    overlaid_loading_table_body(),
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

  let entry_href = OrgUrlHook::new(entry.org).entry_url(entry.id);

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
        a href=(entry_href) class="text-link" {
          code title=(full_path) { (abbreviated_path) }
        }
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

fn empty_body() -> Markup {
  overlaid_message_table_body(
    html! { "Looks like you don't have any entries." },
    html! { "Upload an entry from the CLI to get started." },
  )
}

fn error_body() -> Markup {
  overlaid_critical_message_table_body(
    html! {
      "Failed to load entries."
    },
    html! {},
  )
}
