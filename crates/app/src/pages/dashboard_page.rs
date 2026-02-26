use axum::{Router, response::IntoResponse, routing::get};
use grid_state::AppState;
use maud::html;

use crate::{
  ctx::{RequireAuth, ResponseSeed},
  page_wrapper::page_wrapper,
  pages::util_pages::unauthorized_page,
};

async fn dashboard_page(
  ResponseSeed(ctx, resp): ResponseSeed<RequireAuth>,
) -> impl IntoResponse {
  let auth_state = ctx.auth_state();
  let Some((requested_org_url_hook, _requested_org_hook)) =
    auth_state.requested_org
  else {
    return resp.into_stream(unauthorized_page(ctx.into()));
  };

  let page = html! {
    p class="title" { "You found the dashboard" }
    p { "Requested org: " (requested_org_url_hook.id()) }
  };

  let document = page_wrapper(page, ctx.into());
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/", get(dashboard_page))
}
