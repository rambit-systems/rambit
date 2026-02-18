use maud::{Markup, html};

use crate::ctx::Ctx;

pub(super) fn header(ctx: Ctx) -> Markup {
  const CLASS: &str = "elevation-navbar flex flex-row px-4 gap-2 items-center \
                       h-14 rounded-bl rounded-br mb-8";

  html! {
    div class=(CLASS) {
      (header_logo(ctx.clone()))
      div class="flex-1" {}
      (header_user_area(ctx))
    }
  }
}

fn header_logo(ctx: Ctx) -> Markup {
  const CLASS: &str =
    "cursor-pointer font-display font-bold text-xl text-product-11";

  let href = match ctx.active_org() {
    Some(ao) => ao.dashboard_url(),
    None => "/".to_owned(),
  };

  html! {
    a href=(href) class=(CLASS) {
      "Rambit"
    }
  }
}

fn header_user_area(ctx: Ctx) -> Markup {
  match ctx.auth_user() {
    Some(auth_user) => {
      html! {
        div class="flex flex-col" {
          p { "This still needs to be filled in" }
          p { "User: " (auth_user.id) }
        }
      }
    }
    None => {
      const LOGIN_URL: &str = "/auth/login";
      const SIGNUP_URL: &str = "/auth/signup";
      html! {
        div class="flex flex-row gap-1 items-center" {
          a href=(LOGIN_URL) class="btn-link btn-link-secondary" {
            "Log In"
          }
          a href=(SIGNUP_URL) class="btn-link btn-link-primary" {
            "Sign Up"
          }
        }
      }
    }
  }
}
