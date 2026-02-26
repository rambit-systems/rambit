use const_format::concatcp;
use maud::{Markup, html};

use crate::{
  APP_PREFIX,
  components::text_data::org_descriptor,
  ctx::{Ctx, RequireRequestedOrg},
};

pub(super) fn requested_org_tile(
  ctx: Ctx<RequireRequestedOrg>,
  class: Option<impl AsRef<str>>,
) -> Markup {
  const CLASS: &str = "p-4 elevation-flat flex flex-col gap-4";

  let class = class.as_ref().map(|a| a.as_ref()).unwrap_or_default();
  let class = [CLASS, class].join(" ");

  let requested_org_id = ctx.requested_org_url_hook().id();

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
          (org_descriptor(ctx.into(), requested_org_id))
        }
      }
    }
  }
}
