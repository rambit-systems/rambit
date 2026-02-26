use axum::{Router, response::IntoResponse, routing::get};
use grid_state::AppState;
use maud::{PreEscaped, html};

use crate::{
  ctx::{RequireRequestedOrg, ResponseSeed},
  hooks::OrgHook,
  page_wrapper::page_wrapper,
};

async fn dashboard_page(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  let descriptor_suspense = ctx.suspend(
    move |ctx| async move {
      let meta = ctx.state().domain.meta();
      match meta
        .fetch_org_by_id(ctx.requested_org_url_hook().id())
        .await
      {
        Ok(Some(org)) => {
          PreEscaped(OrgHook::new(org, ctx.auth_user()).descriptor())
        }
        Ok(None) => html! { "[unknown]" },
        Err(_) => html! { "[error]" },
      }
    },
    html! { "[loading]" },
  );

  let page = html! {
    div class="flex flex-col gap-2" {
      p class="title" { "You found the dashboard" }
      p { "Requested org" }
      ul class="list-disc list-inside" {
        li { "ID: " (ctx.requested_org_url_hook().id()) }
        li { "Descriptor: " (descriptor_suspense) }
      }
    }
  };

  let document = page_wrapper(page, ctx.into());
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/", get(dashboard_page))
}
