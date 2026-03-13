//! Store table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use maud::{Markup, html};
use models::{
  LocalStorageCredentials, MemoryStorageCredentials, R2StorageCredentials,
  StorageCredentials, Store,
};

use crate::{
  components::{
    icons::loading_circle,
    table::{
      overlaid_critical_message_table_body, overlaid_loading_table_body,
      overlaid_message_table_body,
    },
    text_data::store_entry_count,
  },
  ctx::{Ctx, RequireAuth, RequireRequestedOrg, ResponseSeed},
  resources::fetch_stores_for_requested_org,
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
          div class="table-cell" { "Storage Type" }
          div class="table-cell" { "Entry Count" }
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
      match ctx
        .fetch_cached(fetch_stores_for_requested_org, ())
        .await
        .as_ref()
      {
        Ok(stores) if stores.is_empty() => empty_body(ctx),
        Ok(stores) => store_rows(ctx.clone().into(), stores.clone()),
        Err(_) => error_body(),
      }
    },
    overlaid_loading_table_body(),
  );

  html! { (suspense) }
}

fn store_rows(ctx: Ctx<RequireAuth>, stores: Vec<Store>) -> Markup {
  html! {
    @for store in stores {
      (store_row(ctx.clone(), store))
    }
  }
}

fn store_row(ctx: Ctx<RequireAuth>, store: Store) -> Markup {
  let storage_type = storage_type_label(&store.credentials);

  html! {
    div class="table-row" {
      div class="table-cell" { code { (store.name.as_ref()) } }
      div class="table-cell" { (storage_type) }
      div class="table-cell" { (store_entry_count(ctx, store.id)) }
    }
  }
}

fn storage_type_label(creds: &StorageCredentials) -> String {
  match creds {
    StorageCredentials::R2(R2StorageCredentials::Default {
      bucket, ..
    }) => format!("R2 ({bucket})"),
    StorageCredentials::Memory(MemoryStorageCredentials) => {
      "Memory (DEBUG)".into()
    }
    StorageCredentials::Local(LocalStorageCredentials(path)) => {
      format!("Local (DEBUG, \"{}\")", path.display())
    }
  }
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
