//! Store table component and HTMX infill handler for the dashboard.

use axum::response::IntoResponse;
use domain::db::DatabaseError;
use maud::{Markup, html};
use models::{
  LocalStorageCredentials, MemoryStorageCredentials, PvR2StorageCredentials,
  PvStorageCredentials, PvStore,
};

use crate::{
  components::icons::loading_circle,
  ctx::{Ctx, RequireRequestedOrg, ResponseSeed},
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

      a href=(create_url) class="btn btn-secondary" { "Create..." }
    }

    div class="w-full overflow-x-auto" {
      div class="table w-full" {
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
      match fetch_stores(ctx).await {
        Ok(stores) if stores.is_empty() => table_empty_body(3),
        Ok(stores) => store_rows(stores),
        Err(_) => table_error_body(3),
      }
    },
    table_placeholder_rows(3, 3),
  );

  html! { (suspense) }
}

async fn fetch_stores(
  ctx: Ctx<RequireRequestedOrg>,
) -> Result<Vec<(PvStore, u64)>, DatabaseError> {
  let org_id = ctx.requested_org_url_hook().id();
  let meta = ctx.state().domain.meta();

  let store_ids = meta.fetch_stores_by_org(org_id).await.inspect_err(|e| {
    tracing::error!("failed to fetch stores by org: {e}");
  })?;

  let mut result = Vec::with_capacity(store_ids.len());
  for store_id in store_ids {
    let Some(store) =
      meta.fetch_store_by_id(store_id).await.inspect_err(|e| {
        tracing::error!("failed to fetch store {store_id}: {e}")
      })?
    else {
      continue;
    };
    let count = meta
      .count_entries_in_store(store_id)
      .await
      .inspect_err(|e| {
        tracing::error!("failed to count entries in store {store_id}: {e}");
      })
      .unwrap_or(0);
    result.push((store.into(), count));
  }

  Ok(result)
}

fn store_rows(stores: Vec<(PvStore, u64)>) -> Markup {
  html! {
    @for (store, count) in stores {
      (store_row(store, count))
    }
  }
}

fn store_row(store: PvStore, entry_count: u64) -> Markup {
  let storage_type = storage_type_label(&store.credentials);

  html! {
    div class="table-row" {
      div class="table-cell font-mono" { (store.name.as_ref()) }
      div class="table-cell" { (entry_count.to_string()) }
      div class="table-cell" { (storage_type) }
    }
  }
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
