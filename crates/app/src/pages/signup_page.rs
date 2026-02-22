use axum::{Router, response::IntoResponse, routing::get};
use grid_state::AppState;
use maud::html;

use crate::{
  components::{form_layout::*, icons::*},
  ctx::ResponseSeed,
  page_wrapper::page_wrapper,
};

const NAME_FIELD_NAME: &str = "name";
const EMAIL_FIELD_NAME: &str = "email";
const PASSWORD_FIELD_NAME: &str = "password";
const CONFIRM_FIELD_NAME: &str = "confirm";

async fn signup_page(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";
  let page = html! {
    form
      class=(FORM_CLASS)
      hx-get="/auth/signup/action"
      hx-target="#form-result"
      hx-swap="innerHTML transition:true"
    {
      (grid_row_full(html! {
        div class="flex flex-col gap-2" {
          p class="title" { "Sign Up" }
          p class="max-w-prose" {
            "Thanks so much for trying us out — we can't wait to get you up and \
            running in no time. Prepare for magical iteration cycle times."
          }
        }
      }))

      (grid_row(html! {
        (grid_row_label(
          "Your name",
          "What do you like to be called?"
        ))

        label class="input-field" {
          div class="size-6 shrink-0" { (user_hero_icon()) }
          input
            class="w-full py-2 focus-visible:outline-none"
            type="text" autofocus="true" required
            placeholder="Full Name" name=(NAME_FIELD_NAME);
        }
      }))

      (grid_row(html! {
        (grid_row_label(
          "Your email",
          "What do you like to be called?"
        ))

        label class="input-field" {
          div class="size-6 shrink-0" { (envelope_hero_icon()) }
          input
            class="w-full py-2 focus-visible:outline-none" required
            type="email" placeholder="Email Address" name=(EMAIL_FIELD_NAME);
        }
      }))

      (grid_row(html! {
        (grid_row_label(
          "Pick a password",
          "[Helpful description goes here]"
        ))

        div class="flex flex-col gap-1" {
          label class="input-field" {
            div class="size-6 shrink-0" { (lock_closed_hero_icon()) }
            input
              class="w-full py-2 focus-visible:outline-none" required
              type="password" placeholder="Password" name=(PASSWORD_FIELD_NAME);
          }
          label class="input-field" {
            div class="size-6 shrink-0" { (lock_closed_hero_icon()) }
            input
              class="w-full py-2 focus-visible:outline-none" required
              type="password" placeholder="Confirm Password" name=(CONFIRM_FIELD_NAME);
            }
        }
      }))

      (grid_row(html! {
        div {}
        div class="flex flex-col gap-4" {
          label {
            input type="submit" class="hidden";
            button class="btn btn-primary w-full max-w-80 justify-between" {
              div class="size-4" {}
              "Sign Up"
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

  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/", get(signup_page))
}
