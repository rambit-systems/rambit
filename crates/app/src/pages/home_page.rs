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
    div class="elevation-flat text-base-12" {
      div class="p-6 sm:p-20 font-semibold flex flex-col gap-2" {
        p class="text-sm sm:text-lg text-product-11 uppercase" {
          "Welcome to Rambit Labs"
        }
        div class="text-4xl sm:text-6xl font-display tracking-tight" {
          p class="font-thin" {
            "Integrate and"
          }
          p { "never waste" }
          p {
            "another "
            span class="text-product-11" { "build" }
          }
        }
      }
    }
  };

  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

pub fn sub_router() -> MethodRouter<AppState, Infallible> { get(home_page) }
