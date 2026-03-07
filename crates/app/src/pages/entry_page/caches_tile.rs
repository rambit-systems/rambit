use maud::{Markup, Render, html};
use models::{Cache, Entry, RecordId, StorePath, Visibility};

use crate::{
  components::icons::lock_closed_hero_icon,
  ctx::{Ctx, RequireAuth},
  indicators,
  resources::fetch_cache,
};

pub fn caches_tile(ctx: Ctx<RequireAuth>, entry: &Entry) -> Markup {
  let caches = entry.caches.clone();
  let store_path = entry.store_path.clone();

  html! {
    div class="flex-1 p-6 elevation-flat flex flex-col gap-4" {
      div class="flex flex-col gap-1" {
        p class="subtitle" { "Resident Caches" }
        p class="max-w-prose" {
          "These are the caches that the entry is attached to."
        }
      }

      div class="table" {
        div class="table-header-group" {
          div class="table-row" {
            div class="table-cell" { "Name" }
            div class="table-cell" { "Download URL" }
            div class="table-cell" { "Visibility" }
          }
        }
        div class="table-row-group" {
          @for cache_id in &caches {
            (cache_row(ctx.clone(), *cache_id, store_path.clone()))
          }
        }
      }
    }
  }
}

fn cache_row(
  ctx: Ctx<RequireAuth>,
  cache_id: RecordId<Cache>,
  store_path: StorePath<String>,
) -> Markup {
  ctx
    .suspend(
      move |ctx| async move {
        match ctx.fetch_cached(fetch_cache, cache_id).await.as_ref() {
          Ok(Some(cache)) => {
            cache_data_row(cache.clone(), store_path)
          }
          Ok(None) => indicators::missing(),
          Err(_) => indicators::error(),
        }
      },
      html! {
        div class="table-row" {
          div class="table-cell py-2" colspan="3" {
            div class="h-4 rounded bg-base-4 animate-pulse" {}
          }
        }
      },
    )
    .render()
}

fn cache_data_row(cache: models::Cache, store_path: StorePath<String>) -> Markup {
  let download_url = format!(
    "/api/v1/c/{cache_name}/download/{store_path}",
    cache_name = cache.name,
  );
  let is_private = matches!(cache.visibility, Visibility::Private);

  html! {
    div class="table-row" {
      div class="table-cell" {
        code { (cache.name.as_ref()) }
      }
      div class="table-cell" {
        a href=(download_url) class="text-link text-link-primary" {
          "Download"
        }
      }
      div class="table-cell" {
        div class="flex flex-row items-center gap-1" {
          (cache.visibility.to_string())
          @if is_private {
            div class="size-4 stroke-base-11/75 stroke-[2.0]" {
              (lock_closed_hero_icon())
            }
          }
        }
      }
    }
  }
}
