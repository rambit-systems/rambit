use domain::db::DatabaseError;
use maud::{Markup, PreEscaped, html};
use models::{Entry, RecordId};

use crate::{
  components::icons::loading_circle,
  ctx::{Ctx, RequireRequestedOrg},
};

pub(super) fn entry_table(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let table_infill_url = ctx
    .requested_org_url_hook()
    .dashboard_entry_table_infill_url();

  html! {
    div class="flex flex-row items-center gap-2" {
      p class="title" { "Entries" }
      div class="flex-1" {}

      button
        class="btn btn-secondary relative overflow-hidden"
        hx-get=(table_infill_url)
        hx-target="#entry-table"
      {
        "Refresh"
        div class="absolute inset-0 flex flex-row justify-center items-center btn-secondary htmx-indicator" {
          div class="size-4" { (loading_circle()) }
        }
      }
    }

    div class="w-full overflow-x-auto" {
      div class="table" {
        div class="table-header-group" {
          div class="table-row" {
            div class="table-cell" { "Store Path" }
            div class="table-cell" { "Caches" }
            div class="table-cell" { "File Size" }
            div class="table-cell" { "Ref Count" }
          }
        }
        div id="entry-table" class="contents" {
          (entry_table_data(ctx))
        }
      }
    }
  }
}

fn table_body_overlay(title: Markup, subtitle: Option<Markup>) -> Markup {
  const INNER_CLASS: &str = "absolute inset-0 bg-base-1 flex flex-col \
                             items-center justify-center border-[2px] \
                             box-border border-base-6 border-dashed rounded-b";

  html! {
    div class="table-row" {
      @ for _ in 0..4 { div class="table-cell" {} }
    }
    div class="table-row" {
      @ for _ in 0..4 { div class="table-cell" {} }
    }
    div class=(INNER_CLASS) {
      p class="text-base-12 text-lg" { (title) }
      @match subtitle {
        Some(subtitle) => {
          p class="text-sm" { (subtitle) }
        }
        None => {}
      }
    }
  }
}

async fn fetch_entry_ids_for_org(
  ctx: Ctx<RequireRequestedOrg>,
) -> Result<Vec<RecordId<Entry>>, DatabaseError> {
  let org_id = ctx.requested_org_url_hook().id();
  let meta = ctx.state().domain.meta();

  let ids = meta.fetch_entries_by_org(org_id).await.inspect_err(|e| {
    tracing::error!("failed to fetch entries by org: {e}");
  })?;

  Ok(ids)
}

fn entry_table_data(ctx: Ctx<RequireRequestedOrg>) -> Markup {
  let render = {
    let ctx = ctx.clone();
    move |entry_ids: Vec<RecordId<Entry>>| {
      if entry_ids.is_empty() {
        return table_body_overlay(
          html! { "Looks like you don't have any entries." },
          Some(html! { "Upload some entries from the CLI to see them here." }),
        );
      }

      html! {
        @for entry_id in entry_ids {
          (entry_row(ctx.clone(), entry_id))
        }
      }
    }
  };
  let placeholder = table_body_overlay(
    html! {
      div class="flex flex-row gap-2 items-center" {
        "Loading"
        div class="size-6" { (loading_circle()) }
      }
    },
    None,
  );
  let suspense = ctx.suspend(
    move |ctx| async move {
      fetch_entry_ids_for_org(ctx.clone()).await.map_or_else(
        |_| {
          table_body_overlay(
            html! { "Failed to load entries" },
            Some(html! { "This is pretty embarrassing..." }),
          )
        },
        render,
      )
    },
    placeholder,
  );

  html! {
    div class="table-row-group min-h-10 relative" {
      (suspense)
    }
  }
}

fn entry_row(
  _ctx: Ctx<RequireRequestedOrg>,
  entry_id: RecordId<Entry>,
) -> Markup {
  html! {
    div class="table-row" {
      (PreEscaped(entry_id.to_string()))
    }
  }
}
