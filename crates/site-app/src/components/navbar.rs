mod account_menu;
mod org_selector;

use leptos::{either::Either, prelude::*};
use models::AuthUser;

use self::{account_menu::AccountMenu, org_selector::OrgSelector};
use crate::{hooks::OrgHook, navigation::next_url_encoded_hook};

#[component]
pub fn Navbar() -> impl IntoView {
  let class = "elevation-navbar flex flex-row px-4 gap-2 items-center h-14 \
               rounded-bl rounded-br mb-8";

  view! {
    <div class=class >
      <NavbarLogo />
      <div class="flex-1" />
      <NavbarUserArea />
    </div>
  }
}

#[component]
fn NavbarLogo() -> impl IntoView {
  let href = match use_context::<AuthUser>() {
    Some(_) => OrgHook::new_active().dashboard_url(),
    None => Memo::new(move |_| "/".to_owned()),
  };

  view! {
    <a href=href class="cursor-pointer font-display font-bold text-xl text-product-11">
      "Rambit"
    </a>
  }
}

#[component]
fn NavbarUserArea() -> impl IntoView {
  let auth_user = use_context::<AuthUser>();

  match auth_user {
    Some(_) => Either::Left(view! { <LoggedInUserAuthActions /> }),
    None => Either::Right(view! { <LoggedOutUserAuthActions /> }),
  }
}

#[component]
fn LoggedOutUserAuthActions() -> impl IntoView {
  let next_url = next_url_encoded_hook();
  let signup_url =
    Signal::derive(move || format!("/auth/signup?next={}", next_url()));
  let login_url =
    Signal::derive(move || format!("/auth/login?next={}", next_url()));

  view! {
    <div class="flex flex-row gap-1 items-center">
      <a href=login_url class="btn-link btn-link-secondary">"Log In"</a>
      <a href=signup_url class="btn-link btn-link-primary">"Sign Up"</a>
    </div>
  }
}

#[component]
fn LoggedInUserAuthActions() -> impl IntoView {
  let active_org_hook = OrgHook::new_active();
  let active_org_dashboard_url = active_org_hook.dashboard_url();

  view! {
    <a href=active_org_dashboard_url class="btn-link btn-link-primary">"Dashboard"</a>
    <OrgSelector />
    <AccountMenu />
    // <a href="/auth/logout" class="btn-link btn-link-secondary">"Log Out"</a>
  }
}
