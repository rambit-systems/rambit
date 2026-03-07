mod action;
mod validate;

use axum::{
  Router,
  response::IntoResponse,
  routing::{get, post},
};
use const_format::concatcp;
use grid_state::AppState;
use maud::html;

use crate::{
  APP_PREFIX,
  components::{form_layout::*, icons::*},
  ctx::{RequireAuth, ResponseSeed},
  extractors::NAME_FIELD_NAME,
  page_wrapper::page_wrapper,
};

const ORG_DESCRIPTION: &str =
  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
   tempor incididunt ut labore et dolore magna aliqua.

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
   aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
   voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

async fn create_org_page(
  ResponseSeed(ctx, resp): ResponseSeed<RequireAuth>,
) -> impl IntoResponse {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";
  const ACTION_URL: &str = concatcp!(APP_PREFIX, "/org/create_org/action");
  const VALIDATION_URL: &str =
    concatcp!(APP_PREFIX, "/org/create_org/validate");

  let page = html! {
    form
      class=(FORM_CLASS)
      hx-post=(ACTION_URL)
      hx-target="#form-result"
    {
      (grid_row_full(html! {
        div class="flex flex-col gap-2" {
          p class="title" { "Create an Organization" }
          p class="max-w-prose whitespace-pre-line" { (ORG_DESCRIPTION) }
        }
      }))

      (grid_row_full(html! {
        div class="h-0 border-t-[1.5px] border-base-6 w-full" {}
      }))

      (grid_row(html! {
        (grid_row_label(
          "Org name",
          "This will act as your namespace."
        ))

        div class="flex flex-col gap-1" {
          label class="input-field" {
            div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (building_office_2()) }
            input
              class="w-full py-2 focus-visible:outline-none"
              type="text" autofocus="true" required
              placeholder="Org Name" name=(NAME_FIELD_NAME)

              hx-get=(VALIDATION_URL)
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
        div {}
        div class="flex flex-col gap-4" {
          label {
            input type="submit" class="hidden";
            button class="btn btn-primary w-full max-w-80 justify-between" {
              div class="size-4" {}
              "Create Org"
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
    .route("/", get(create_org_page))
    .route("/validate", get(self::validate::validate_org_name))
    .route("/action", post(self::action::signup_action))
}
