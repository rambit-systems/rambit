use axum::{Router, response::IntoResponse, routing::get};
use const_format::concatcp;
use grid_state::AppState;
use maud::html;

use crate::{
  APP_PREFIX, components::icons::*, ctx::ResponseSeed,
  page_wrapper::page_wrapper,
};

const EMAIL_FIELD_NAME: &str = "email";
const PASSWORD_FIELD_NAME: &str = "password";

async fn login_page(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  const FORM_CLASS: &str =
    "p-8 self-stretch sm:self-center w-auto elevation-flat";
  const ACTION_URL: &str = concatcp!(APP_PREFIX, "/auth/login/action");

  let page = html! {
    div class="p-8 self-stretch sm:self-center w-auto elevation-flat" {
      form
        class="max-w-80 flex flex-col gap-6"
        hx-get="/auth/login/action"
        hx-target="#form-result"
        hx-swap="innerHTML transition:true"
      {
        p class="title" { "Login" }
        p class="max-w-prose" {
          "Welcome back to the most satisfying part of your CI/CD pipeline."
        }

        div class="flex flex-col gap-4" {
          // email
          label class="flex flex-col gap-1" {
            p class="text-base-11" { "Email Address" }
            div class="input-field" {
              div class="size-6 shrink-0" { (envelope_hero_icon()) }
              input
                class="w-full py-2 focus-visible:outline-none" required
                type="email" autofocus="true" placeholder="" name=(EMAIL_FIELD_NAME);
            }
          }

          // password
          label class="flex flex-col gap-1" {
            p class="text-base-11" { "Password" }
            div class="input-field" {
              div class="size-6 shrink-0" { (lock_closed_hero_icon()) }
              input
                class="w-full py-2 focus-visible:outline-none" required
                type="password" placeholder="" name=(PASSWORD_FIELD_NAME);
            }
          }
        }

        // submit
        div class="flex flex-col gap-4" {
          label {
            input type="submit" class="hidden";
            button class="btn btn-primary w-full justify-between" {
              div class="size-4" {}
              "Log in to Rambit"
              div class="size-4 transition-opacity htmx-indicator" {
                (loading_circle())
              }
            }
          }
          div id="form-result" class="contents" {}
        }
      }
    }
  };

  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/", get(login_page))
  // .route("/action", post(self::action::signup_action))
}
