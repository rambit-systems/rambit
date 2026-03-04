//! The dashboard page and its HTMX infill sub-routes.

mod cache_table;
mod entry_table;
mod requested_org_tile;
mod store_table;

use axum::{Router, response::IntoResponse, routing::get};
use grid_state::AppState;
use maud::html;

use self::requested_org_tile::requested_org_tile;
use crate::{
  ctx::{RequireRequestedOrg, ResponseSeed},
  page_wrapper::page_wrapper,
};

async fn dashboard_page(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  const OUTER_CLASS: &str = "flex flex-col md:grid \
                             xl:grid-cols-[320px_minmax(0,_1fr)] gap-4 \
                             md:place-items-start";
  const TABLE_CONTAINER_CLASS: &str = "flex-1 md:col-span-2 xl:col-span-1 \
                                       flex flex-col md:grid gap-4 \
                                       md:grid-cols-2 md:place-self-stretch";

  let page = html! {
    div class=(OUTER_CLASS) {
      p class="title xl:col-start-2" { "Dashboard" }

      (requested_org_tile(ctx.clone(), Some("md:place-self-end xl:place-self-auto md:w-80")))

      div class=(TABLE_CONTAINER_CLASS) {
        div class="p-6 elevation-flat flex flex-col gap-4" {
          (self::cache_table::cache_table(ctx.clone()))
        }
        div class="p-6 elevation-flat flex flex-col gap-4" {
          (self::store_table::store_table(ctx.clone()))
        }
        div class="col-span-2 p-6 elevation-flat flex flex-col gap-4" {
          (self::entry_table::entry_table(ctx.clone()))
        }
      }
    }
  };

  let document = page_wrapper(page, ctx.into());
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new()
    .route("/", get(dashboard_page))
    .route("/cache_table", get(self::cache_table::cache_table_infill))
    .route("/store_table", get(self::store_table::store_table_infill))
    .route("/entry_table", get(self::entry_table::entry_table_infill))
}
