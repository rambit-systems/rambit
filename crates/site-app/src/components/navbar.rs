use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
  let class = "elevation-navbar flex flex-row px-4 gap-1 items-center h-10 \
               rounded-bl rounded-br mb-8";

  view! {
    <div class=class >
      <span class="font-display font-bold text-xl text-product-11">
        "Rambit"
      </span>
      <div class="flex-1" />
      <a href="/auth/login" class="btn-link btn-link-secondary">Login</a>
      <a href="/auth/signup" class="btn-link btn-link-primary">Sign Up</a>
    </div>
  }
}
