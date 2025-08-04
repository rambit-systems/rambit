use leptos::{either::Either, prelude::*};
use leptos_router::location::Url;
use models::{AuthStatus, AuthUser};

use crate::navigation::url_to_full_path;

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
  let query = leptos_router::hooks::use_query_map();
  // if we already have a `next` url
  let existing_next_url = Signal::derive(move || query().get("next"));

  // if we need a `next_url`
  let return_url = leptos_router::hooks::use_url();
  let escaped_return_url =
    Signal::derive(move || Url::escape(&url_to_full_path(&return_url())));

  // use the existing `next` url if it exists, rather than setting it to the
  // current page
  let redirect_url = Signal::derive(move || match existing_next_url() {
    Some(existing_next_url) => existing_next_url,
    _ => escaped_return_url(),
  });

  let signup_url =
    Signal::derive(move || format!("/auth/signup?next={}", redirect_url()));
  let login_url =
    Signal::derive(move || format!("/auth/login?next={}", redirect_url()));

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
