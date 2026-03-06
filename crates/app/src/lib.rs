//! Contains the app code for the Rambit app.

mod components;
mod ctx;
mod extractors;
mod form_feedback_text;
mod hooks;
mod indicators;
mod internal_error;
mod page_wrapper;
mod pages;
mod request_cache;
mod resources;

use axum::{Router, response::IntoResponse};
use grid_state::AppState;
use maud::html;

use self::{ctx::ResponseSeed, page_wrapper::page_wrapper};

/// The prefix for all the app pages.
pub const APP_PREFIX: &str = "/app";

/// The fallback handler.
pub async fn fallback_handler(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  let page = html! {
    h1 { "Page not found." }
  };
  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

/// Builds the app router.
pub fn router() -> Router<AppState> {
  Router::new().nest(APP_PREFIX, app_router())
}

fn app_router() -> Router<AppState> {
  Router::new()
    .merge(pages::home_page::sub_router())
    .nest(
      "/auth",
      Router::new()
        .nest("/signup", pages::signup_page::sub_router())
        .nest("/login", pages::login_page::sub_router()),
    )
    .nest("/org_selector", components::org_selector::sub_router())
    .nest(
      "/org",
      Router::new()
        .nest("/create_org", pages::create_org_page::sub_router())
        .nest(
          "/{org}",
          Router::new()
            .nest("/dash", pages::dashboard_page::sub_router())
            .nest("/create_cache", pages::create_cache_page::sub_router())
            .nest("/create_store", pages::create_store_page::sub_router()),
        ),
    )
    .nest("/component-zoo", pages::zoo_page::sub_router())
}
