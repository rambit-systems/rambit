//! The "create a cache" page and its sub-routes.

mod action;
mod validate;
mod visibility_selector;

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

const CACHE_DESCRIPTION: &str =
  "A cache is a container and access-control mechanism for entries, and is \
   the primary namespace through which users will consume your entries.

A cache's name must be globally unique (across organizations), even if the \
   cache is set to private. The visibility of the cache controls whether its \
   entries are accessible outside of your organization.

   Generally cache names are on a first-come-first-served basis, but please \
   contact us if you have concerns.";

async fn create_cache_page(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  let org_id = ctx.requested_org_url_hook().id();
  let org_hook = crate::hooks::OrgUrlHook::new(org_id);
  let action_url = org_hook.create_cache_action_url();
  let validation_url = org_hook.create_cache_validation_url();

  let page = html! {
    form
      class=(FORM_CLASS)
      hx-post=(action_url)
      hx-target="#form-result"
    {
      (grid_row_full(html! {
        div class="flex flex-col gap-2" {
          p class="title" { "Create a Cache" }
          p class="max-w-prose whitespace-pre-line" { (CACHE_DESCRIPTION) }
        }
      }))

      (grid_row_full(html! {
        div class="h-0 border-t-[1.5px] border-base-6 w-full" {}
      }))

      (grid_row(html! {
        (grid_row_label(
          "Cache name",
          "Think of it like a username."
        ))

        div class="flex flex-col gap-1" {
          label class="input-field" {
            div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (archive_box_hero_icon()) }
            input
              class="w-full py-2 focus-visible:outline-none"
              type="text" autofocus="true" required
              placeholder="Cache Name" name=(NAME_FIELD_NAME)
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
          "Visibility",
          "For the public good or just your team?"
        ))

        (self::visibility_selector::visibility_selector())
      }))

      (grid_row(html! {
        div {}
        div class="flex flex-col gap-4" {
          label {
            input type="submit" class="hidden";
            button class="btn btn-primary w-full max-w-80 justify-between" {
              div class="size-4" {}
              "Create Cache"
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
    .route("/", get(create_cache_page))
    .route("/validate", get(self::validate::validate_cache_name))
    .route("/action", post(self::action::create_cache_action))
}
