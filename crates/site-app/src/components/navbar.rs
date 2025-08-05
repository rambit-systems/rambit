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

/// Gets the next URL if it's already set or sets it to the current page.
fn next_url_hook() -> Memo<String> {
  let query = leptos_router::hooks::use_query_map();
  let current_url = leptos_router::hooks::use_url();

  // set it to the existing next url or the current URL escaped
  Memo::new(move |_| {
    query()
      .get("next")
      .unwrap_or(Url::escape(&url_to_full_path(&current_url())))
  })
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
