//! Contains the app code for the Rambit app.

mod ctx;
mod extractors;
mod hooks;
mod internal_error;
mod page_wrapper;

use axum::response::IntoResponse;
use maud::html;

use self::{ctx::ResponseSeed, page_wrapper::page_wrapper};

/// The fallback handler.
pub async fn fallback_handler(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  let page = html! {
    h1 { "We couldn't find that page :/" }
  };
  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}
