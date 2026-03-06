//! Store table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use maud::{Markup, Render, html};
use models::{
  LocalStorageCredentials, MemoryStorageCredentials, PvR2StorageCredentials,
  PvStorageCredentials, PvStore, RecordId, Store,
};

use crate::{
  components::icons::loading_circle,
  ctx::{Ctx, RequireAuth, RequireRequestedOrg, ResponseSeed},
  indicators,
  resources::{AuthResult, fetch_entry_count_for_store, fetch_stores_for_requested_org},
};

/// Renders the full store table card (shell + initial infill).
pub(super) fn store_table(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let org_hook = ctx.requested_org_url_hook();
  let infill_url = org_hook.dashboard_store_table_infill_url();
  let create_url = org_hook.create_store_url();

  html! {
    div class="flex flex-row items-center gap-2" {
      p class="title" { "Stores" }
      div class="flex-1" {}

      button
        class="btn btn-secondary relative overflow-hidden"
        hx-get=(infill_url)
        hx-target="#store-table-body"
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
          div class="table-cell" { "Entry Count" }
          div class="table-cell" { "Storage Type" }
        }
      }
      div id="store-table-body" class="table-row-group" {
        (store_table_data(ctx))
      }
    }
  }
}

/// HTMX infill handler — returns only the row-group contents.
pub(super) async fn store_table_infill(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  resp.into_stream(store_table_data(ctx))
}

fn store_table_data(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let suspense = ctx.suspend(
    move |ctx| async move {
      match ctx.fetch_cached(fetch_stores_for_requested_org, ()).await.as_ref() {
        Ok(stores) if stores.is_empty() => table_empty_body(3),
        Ok(stores) => store_rows(ctx.clone().into(), stores.clone()),
        Err(_) => table_error_body(3),
      }
    },
    table_placeholder_rows(3, 3),
  );

  html! { (suspense) }
}

fn store_rows(ctx: Ctx<RequireAuth>, stores: Vec<PvStore>) -> Markup {
  html! {
    @for store in stores {
      (store_row(ctx.clone(), store))
    }
  }
}

fn store_row(ctx: Ctx<RequireAuth>, store: PvStore) -> Markup {
  let storage_type = storage_type_label(&store.credentials);

  html! {
    div class="table-row" {
      div class="table-cell" { code { (store.name.as_ref()) } }
      div class="table-cell" { (store_entry_count(ctx, store.id)) }
      div class="table-cell" { (storage_type) }
    }
  }
}

fn store_entry_count(ctx: Ctx<RequireAuth>, store_id: RecordId<Store>) -> Markup {
  ctx
    .suspend(
      move |ctx| async move {
        match ctx.fetch_cached(fetch_entry_count_for_store, store_id).await.as_ref() {
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

fn storage_type_label(creds: &PvStorageCredentials) -> String {
  match creds {
    PvStorageCredentials::R2(PvR2StorageCredentials::Default {
      bucket,
      ..
    }) => format!("R2 ({bucket})"),
    PvStorageCredentials::Memory(MemoryStorageCredentials) => {
      "Memory (DEBUG)".into()
    }
    PvStorageCredentials::Local(LocalStorageCredentials(path)) => {
      format!("Local (DEBUG, \"{}\")", path.display())
    }
  }
}

fn table_empty_body(cols: usize) -> Markup {
  html! {
    div class="table-row" {
      div class="table-cell py-4 text-center text-base-11"
          colspan=(cols.to_string())
      {
        "No stores yet."
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
        "Failed to load stores."
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
