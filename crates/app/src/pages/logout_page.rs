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
  components::{
    form_result::form_rejection, icons::loading_circle,
    scripts::redirect_script,
  },
  ctx::{Ctx, ResponseSeed},
  page_wrapper::page_wrapper,
};

const ACTION_URL: &str = concatcp!(APP_PREFIX, "/auth/logout/action");

const DESCRIPTION: &str = "Are you sure you want to log out? We're sad to see \
                           you go but excited for you to come back.";

async fn logout_page(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  const OUTER_CLASS: &str =
    "p-8 self-stretch sm:self-center w-auto elevation-flat";
  const FORM_CLASS: &str = "max-w-80 flex flex-col gap-6";

  let page = html! {
    div class=(OUTER_CLASS) {
      form
        class=(FORM_CLASS)
        hx-post=(ACTION_URL)
        hx-target="#form-result"
      {
        p class="title" { "Log Out" }
        p class="max-w-prose" { (DESCRIPTION) }

        div class="flex flex-col gap-4" {
          label {
            input type="submit" class="hidden";
            button class="btn btn-critical-subtle w-full max-w-80 justify-between" {
              div class="size-4" {}
              "Log Out"
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
