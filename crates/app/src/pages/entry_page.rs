//! The entry detail page.

mod action_tile;
mod caches_tile;
mod store_path_tile;
mod title_tile;

use std::{collections::HashMap, str::FromStr};

use axum::{
  Router,
  extract::Path,
  response::IntoResponse,
  routing::{get, post},
};
use grid_state::AppState;
use maud::{Markup, html};
use models::{Entry, RecordId};

use self::{
  action_tile::action_tile, caches_tile::caches_tile,
  store_path_tile::store_path_tile, title_tile::title_tile,
};
use crate::{
  ctx::{Ctx, RequireAuth, RequireRequestedOrg, ResponseSeed},
  hooks::OrgUrlHook,
  page_wrapper::page_wrapper,
  resources::fetch_entry,
};

pub fn sub_router() -> Router<AppState> {
  Router::new()
    .route("/", get(entry_page))
    .route("/delete", post(action_tile::delete_entry_action))
}

async fn entry_page(
  Path(params): Path<HashMap<String, String>>,
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  let entry_id = match params
    .get("entry")
    .and_then(|s| RecordId::<Entry>::from_str(s).ok())
  {
    Some(id) => id,
    None => {
      let page = missing_entry();
      let document = page_wrapper(page, ctx.into());
      return resp.into_stream(document);
    }
  };

  let org_id = ctx.requested_org_url_hook().id();
  let org_url_hook = OrgUrlHook::new(org_id);
  let auth_ctx = ctx.clone().into_require_auth();

  let entry_result = auth_ctx.fetch_cached(fetch_entry, entry_id).await;

  let page = match entry_result.as_ref() {
    Ok(Some(entry)) if entry.org == org_id => {
      entry_inner(ctx.clone().into_require_auth(), &org_url_hook, entry)
    }
    Ok(_) => missing_entry(),
    Err(_) => error_entry(),
  };

  let document = page_wrapper(page, ctx.into());
  resp.into_stream(document)
}

fn entry_inner(
  ctx: Ctx<RequireAuth>,
  org_url_hook: &OrgUrlHook,
  entry: &Entry,
) -> Markup {
  const OUTER_CLASS: &str =
    "flex flex-col md:grid md:grid-cols-[max-content_auto] gap-4 p-4";

  html! {
    div class=(OUTER_CLASS) {
      // hidden spacer (md:block only), to push title to second column
      div class="hidden md:block" {}

      // title tile spans full row on md+
      div class="md:col-span-1" {
        (title_tile(&entry.store_path))
      }

      // action tile - left column
      (action_tile(org_url_hook, entry.id))

      // right column - store path and caches tiles
      div class="flex flex-row gap-4 flex-wrap" {
        (store_path_tile(&entry.store_path))
        (caches_tile(ctx, entry))
      }
    }
  }
}

fn missing_entry() -> Markup {
  html! {
    div class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8" {
      p class="title" { "Entry Not Found" }
      p { "This entry does not exist or you do not have access to it." }
    }
  }
}

fn error_entry() -> Markup {
  html! {
    div class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8" {
      p class="title" { "Error" }
      p { "Failed to load entry." }
    }
  }
}
