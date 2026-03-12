//! The "create a store" page and its sub-routes.

mod action;
mod credentials_input;
mod validate;

use axum::{
  Router,
  response::IntoResponse,
  routing::{get, post},
};
use grid_state::AppState;
use maud::html;

use crate::{
  components::{form_layout::*, icons::*},
  ctx::{RequireRequestedOrg, ResponseSeed},
  extractors::NAME_FIELD_NAME,
  page_wrapper::page_wrapper,
};

const STORE_DESCRIPTION: &str =
  "A store represents a storage location for entries, for example an S3 \
   bucket. The store holds credentials for the storage location, and \
   configuration specifying how the entries it contains will be encoded.

   Stores are immutable aside from their entry list. To change a store's \
   credentials or encoding configuration, you will need to create a new store \
   and migrate the old store's entries to it. This incurs compute costs.";

async fn create_store_page(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  let org_id = ctx.requested_org_url_hook().id();
  let org_hook = crate::hooks::OrgUrlHook::new(org_id);
  let action_url = org_hook.create_store_action_url();
  let validation_url = org_hook.create_store_validation_url();

  let page = html! {
    form
      class=(FORM_CLASS)
      hx-post=(action_url)
      hx-target="#form-result"
    {
      (grid_row_full(html! {
        div class="flex flex-col gap-2" {
          p class="title" { "Create a Store" }
          p class="max-w-prose whitespace-pre-line" { (STORE_DESCRIPTION) }
        }
      }))

      (grid_row_full(html! {
        div class="h-0 border-t-[1.5px] border-base-6 w-full" {}
      }))

      (grid_row(html! {
        (grid_row_label(
          "Store name",
          "Use something memorable."
        ))

        div class="flex flex-col gap-1" {
          label class="input-field" {
            div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (archive_box_hero_icon()) }
            input
              class="w-full py-2 focus-visible:outline-none"
              type="text" autofocus="true" required
              placeholder="Store Name" name=(NAME_FIELD_NAME)
              autocorrect="off" spellcheck="false" autocomplete="off"

              hx-get=(validation_url)
              hx-target="#name-hint"
              hx-trigger="input"
              hx-indicator="next div";

            div class="size-6 shrink-0 transition-opacity htmx-indicator" {
              (loading_circle())
            }
          }
          div id="name-hint" class="contents" {}
        }
      }))

      (grid_row(html! {
        (grid_row_label(
          "Storage credentials",
          "These credentials are for the storage location where your data \
           will sit."
        ))

        (self::credentials_input::credentials_input())
      }))

      (grid_row(html! {
        div {}
        div class="flex flex-col gap-4" {
          label {
            input type="submit" class="hidden";
            button class="btn btn-primary w-full max-w-80 justify-between" {
              div class="size-4" {}
              "Create Store"
              div class="size-4 transition-opacity htmx-indicator" {
                (loading_circle())
              }
            }
          }
          div id="form-result" class="contents" {}
        }
      }))
    }
  };

  let document = page_wrapper(page, ctx.into());
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new()
    .route("/", get(create_store_page))
    .route("/validate", get(self::validate::validate_store_name))
    .route("/action", post(self::action::create_store_action))
}
