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
  components::{form_result::form_rejection, scripts::redirect_script},
  ctx::{Ctx, ResponseSeed},
  page_wrapper::page_wrapper,
};

const ACTION_URL: &str = concatcp!(APP_PREFIX, "/auth/logout/action");

async fn logout_page(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  let page = html! {
    div class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8" {
      p class="title" { "Log out" }

      p class="max-w-prose" {
        "Are you sure you want to log out? We're sad to see you go but excited for you to come back."
      }

      div class="flex flex-row" {
        form
          hx-post=(ACTION_URL)
          hx-target="#logout-result"
        {
          button class="btn btn-critical-subtle w-full max-w-80 justify-between" {
            div class="size-4" {}
            "Log out"
            div class="size-4 transition-opacity htmx-indicator" {
              (crate::components::icons::loading_circle())
            }
          }
        }
      }

      div id="logout-result" class="contents" {}
    }
  };

  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

async fn logout_action(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  let feedback = match perform_logout(ctx).await {
    Ok(()) => html! {
      (redirect_script("/", None))
    },
    Err(e) => form_rejection(e),
  };

  resp.into_stream(feedback)
}

async fn perform_logout(ctx: Ctx) -> Result<(), &'static str> {
  let mut auth_session = ctx.auth_session();
  auth_session.logout().await.map_err(|e| {
    tracing::error!("failed to deauthenticate: {e}");
    "internal error"
  })?;
  Ok(())
}

pub fn sub_router() -> Router<AppState> {
  Router::new()
    .route("/", get(logout_page))
    .route("/action", post(logout_action))
}
