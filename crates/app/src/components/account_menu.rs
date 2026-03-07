use const_format::concatcp;
use maud::{Markup, PreEscaped, html};

use crate::{
  APP_PREFIX,
  components::icons::*,
  ctx::{Ctx, RequireAuth},
};

pub fn account_menu(ctx: Ctx<RequireAuth>) -> Markup {
  const BUTTON_CLASS: &str = concatcp!(
    " btn-secondary rounded-full",
    " cursor-pointer transition select-none button-squish",
    " border-[1.5px] border-base-6",
    " size-10 flex flex-col justify-center items-center",
    " [anchor-name:--account-menu-anchor]",
  );
  const POPOVER_CLASS: &str = concatcp!(
    " min-w-56 elevation-lv1 p-2 rounded",
    " transition transition-discrete duration-200 opacity-0 scale-97",
    " [&:popover-open]:opacity-100 [&:popover-open]:scale-100",
    " [@starting-style]:[&:popover-open]:opacity-0 \
     [@starting-style]:[&:popover-open]:scale-97",
    " [position-anchor:--account-menu-anchor]",
    " top-[anchor(bottom)] left-[anchor(right)]",
    " -translate-x-full translate-y-[calc(var(--spacing)*4)]"
  );
  const ICON_CLASS: &str = "size-5 shrink-0 stroke-base-11 stroke-[2.0]";

  const LOGOUT_HREF: &str = concatcp!(APP_PREFIX, "/auth/logout");

  let name_abbr = ctx.auth_user().name_abbr.to_string();

  html! {
    button class=(BUTTON_CLASS) popovertarget="account-menu-popover" {
      (PreEscaped(name_abbr))
    }

    div popover id="account-menu-popover" class=(POPOVER_CLASS) {
      div class="flex flex-col gap-1 leading-none" {
        a class="btn-link btn-link-secondary btn-link-tight" {
          div class=(ICON_CLASS) { (cog_6_tooth_hero_icon()) }
          "User Settings"
        }
        a href=(LOGOUT_HREF) class="btn-link btn-critical-subtle btn-link-tight" {
          div class=(ICON_CLASS) { (arrow_right_start_on_rectangle_hero_icon()) }
          "Log Out"
        }
      }
    }
  }
}
