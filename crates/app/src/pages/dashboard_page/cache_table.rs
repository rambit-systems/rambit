//! Cache table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use maud::{Markup, Render, html};
use models::{Cache, PvCache, RecordId, Visibility};

use crate::{
  components::{
    icons::*,
    table::{
      overlaid_critical_message_table_body, overlaid_loading_table_body,
      overlaid_message_table_body,
    },
  },
  ctx::{Ctx, RequireAuth, RequireRequestedOrg, ResponseSeed},
  indicators,
  resources::{
    AuthResult, fetch_caches_for_requested_org, fetch_entry_count_for_cache,
  },
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

    div class="table" {
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

/// HTMX infill handler — returns only the row-group contents.
pub(super) async fn cache_table_infill(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  resp.into_stream(cache_table_data(ctx))
}

fn cache_table_data(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let suspense = ctx.suspend(
    move |ctx| async move {
      match ctx
        .fetch_cached(fetch_caches_for_requested_org, ())
        .await
        .as_ref()
      {
        Ok(caches) if caches.is_empty() => empty_body(ctx),
        Ok(caches) => cache_rows(ctx.clone().into(), caches.clone()),
        Err(_) => error_body(),
      }
    },
    overlaid_loading_table_body(),
  );

  html! { (suspense) }
}

fn cache_rows(ctx: Ctx<RequireAuth>, caches: Vec<PvCache>) -> Markup {
  html! {
    @for cache in caches {
      (cache_row(ctx.clone(), cache))
    }
  }
}

fn cache_row(ctx: Ctx<RequireAuth>, cache: PvCache) -> Markup {
  html! {
    div class="table-row" {
      // name
      div class="table-cell" { code { (cache.name.as_ref()) } }

      // visibility
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

      // entry count
      div class="table-cell" {
        (cache_entry_count(ctx, cache.id))
      }
    }
  }
}

fn cache_entry_count(
  ctx: Ctx<RequireAuth>,
  cache_id: RecordId<Cache>,
) -> Markup {
  ctx
    .suspend(
      move |ctx| async move {
        match ctx
          .fetch_cached(fetch_entry_count_for_cache, cache_id)
          .await
          .as_ref()
        {
          Ok(Some(AuthResult::Ok(c))) => html! { (c) },
          Ok(Some(AuthResult::Unauthorized)) => indicators::unauthorized(),
          Ok(None) => indicators::missing(),
          Err(_) => indicators::error(),
        }
      },
      indicators::loading(),
    )
    .render()
}

fn empty_body(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let create_url = ctx.requested_org_url_hook().create_cache_url();

  overlaid_message_table_body(
    html! { "Looks like you don't have any caches." },
    html! {
      a href=(create_url) class="text-link text-link-primary" { "Create one" }
      " to get started."
    },
  )
}

fn error_body() -> Markup {
  overlaid_critical_message_table_body(
    html! {
      "Failed to load caches."
    },
    html! {},
  )
}
