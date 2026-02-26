mod requested_org_tile;

use axum::{Router, response::IntoResponse, routing::get};
use grid_state::AppState;
use maud::{PreEscaped, html};

use self::requested_org_tile::requested_org_tile;
use crate::{
  ctx::{RequireRequestedOrg, ResponseSeed},
  hooks::OrgHook,
  page_wrapper::page_wrapper,
};

async fn dashboard_page(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  let org_id = ctx.requested_org_url_hook().id();
  let descriptor_suspense = ctx.suspend(
    move |ctx| async move {
      match ctx.fetch_org(org_id).await {
        Ok(Some(org)) => {
          PreEscaped(OrgHook::new(org, ctx.auth_user()).descriptor())
        }
        Ok(None) => html! { "[unknown]" },
        Err(_) => html! { "[error]" },
      }
    },
    html! { "[loading]" },
  );

  const OUTER_CLASS: &str = "flex flex-col md:grid \
                             xl:grid-cols-[320px_minmax(0,_1fr)] gap-4 \
                             md:place-items-start";

  let page = html! {
    div class=(OUTER_CLASS) {
      p class="title xl:col-start-2" { "Dashboard" }
      (requested_org_tile(ctx.clone(), Some("md:place-self-end xl:place-self-auto md:w-80")))
    }
  };

  let document = page_wrapper(page, ctx.into());
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/", get(dashboard_page))
}
