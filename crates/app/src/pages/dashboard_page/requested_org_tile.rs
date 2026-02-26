use const_format::concatcp;
use maud::{Markup, PreEscaped, html};

use crate::{
  APP_PREFIX,
  ctx::{Ctx, RequireRequestedOrg},
  hooks::OrgHook,
};

pub(super) fn requested_org_tile(
  ctx: Ctx<RequireRequestedOrg>,
  class: Option<impl AsRef<str>>,
) -> Markup {
  const CLASS: &str = "p-4 elevation-flat flex flex-col gap-4";

  let class = class.as_ref().map(|a| a.as_ref()).unwrap_or_default();
  let class = [CLASS, class].join(" ");

  let requested_org_id = ctx.requested_org_url_hook().id();
  let descriptor_suspense = ctx.suspend(
    move |ctx| async move {
      match ctx.fetch_org(requested_org_id).await {
        Ok(Some(org)) => {
          PreEscaped(OrgHook::new(org, ctx.auth_user()).descriptor())
        }
        Ok(None) => html! { "[unknown]" },
        Err(_) => html! { "[error]" },
      }
    },
    html! { "[loading]" },
  );

  const CREATE_ORG_URL: &str = concatcp!(APP_PREFIX, "/org/create_org");

  html! {
    div class=(class) {
      div class="flex flex-col leading-none" {
        div class="flex flex-row gap-2 items-center justify-between" {
          p class="text-xl" { "org" }
          a
            href=(CREATE_ORG_URL)
            class="text-link text-link-primary"
          { "Create Org..." }
        }
        p class="text-3xl text-base-12" {
          (descriptor_suspense)
        }
      }
    }
  }
}
