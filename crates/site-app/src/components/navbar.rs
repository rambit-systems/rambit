use leptos::{either::Either, prelude::*};
use models::AuthUser;

use crate::{hooks::OrgHook, navigation::next_url_hook};

#[component]
pub fn Navbar() -> impl IntoView {
  let class = "elevation-navbar flex flex-row px-4 gap-2 items-center h-12 \
               rounded-bl rounded-br mb-8";

  view! {
    <div class=class >
      <a href="/" class="cursor-pointer font-display font-bold text-xl text-product-11">
        "Rambit"
      </a>
      <div class="flex-1" />
      <NavbarUserArea />
    </div>
  }
}
#[component]
fn NavbarUserArea() -> impl IntoView {
  let auth_user = use_context::<AuthUser>();

  match auth_user {
    Some(user) => Either::Left(view! { <LoggedInUserAuthActions user=user /> }),
    None => Either::Right(view! { <LoggedOutUserAuthActions /> }),
  }
}

#[component]
fn LoggedOutUserAuthActions() -> impl IntoView {
  let next_url = next_url_hook();
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
fn LoggedInUserAuthActions(user: AuthUser) -> impl IntoView {
  let active_org = user.active_org();
  let org_hook = OrgHook::new(move || active_org, user.clone());
  let descriptor = org_hook.descriptor();
  let dashboard_url = org_hook.dashboard_url();

  view! {
    <div class="flex flex-col gap leading-none items-end">
      <span class="text-base-12">{ user.name.to_string() }</span>
      <span class="text-sm">
        <Suspense fallback=|| "[loading]">
          { move || Suspend::new(descriptor) }
        </Suspense>
      </span>
    </div>
    <div class="flex flex-row gap-1 items-center">
      <a href=dashboard_url class="btn-link btn-link-primary">"Dashboard"</a>
      <a href="/auth/logout" class="btn-link btn-link-secondary">"Log Out"</a>
    </div>
  }
}
