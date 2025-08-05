use leptos::{either::Either, prelude::*};
use models::{AuthStatus, AuthUser};

use crate::navigation::next_url_hook;

#[component]
pub fn Navbar() -> impl IntoView {
  let class = "elevation-navbar flex flex-row px-4 gap-1 items-center h-10 \
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
  let auth_status = use_context::<AuthStatus>().and_then(|as_| as_.0);

  match auth_status {
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
    <a href=login_url class="btn-link btn-link-secondary">"Log In"</a>
    <a href=signup_url class="btn-link btn-link-primary">"Sign Up"</a>
  }
}

#[component]
fn LoggedInUserAuthActions(user: AuthUser) -> impl IntoView {
  view! {
    <span class="">
      "Welcome, "
      <a class="text-link text-link-primary">
        { user.name.to_string() }
      </a>
    </span>
    <a href="/auth/logout" class="btn-link btn-link-secondary">"Log Out"</a>
  }
}
