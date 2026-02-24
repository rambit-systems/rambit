use axum::{Router, response::IntoResponse, routing::get};
use grid_state::AppState;
use maud::html;

use crate::{
  ctx::ResponseSeed, page_wrapper::page_wrapper,
  pages::util_pages::unauthorized_page,
};

async fn dashboard_page(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  let Some(auth_state) = ctx.auth_state() else {
    return resp.into_stream(unauthorized_page(ctx));
  };
  let Some((requested_org_url_hook, _requested_org_hook)) =
    auth_state.requested_org
  else {
    return resp.into_stream(unauthorized_page(ctx));
  };

  let page = html! {
    p class="title" { "You found the dashboard" }
    p { "Requested org: " (requested_org_url_hook.id()) }
  };

  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/", get(dashboard_page))
}
