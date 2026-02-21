use std::convert::Infallible;

use axum::{
  response::IntoResponse,
  routing::{MethodRouter, get},
};
use grid_state::AppState;
use maud::html;

use crate::{ctx::ResponseSeed, page_wrapper::page_wrapper};

pub async fn home_page(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  let page = html! {
    p class="title" {
      "Welcome to the home page"
    }
  };

  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

pub fn sub_router() -> MethodRouter<AppState, Infallible> { get(home_page) }
