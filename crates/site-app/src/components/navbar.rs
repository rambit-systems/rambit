use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
  let class = "bg-product-1 flex flex-row px-4 gap-1 items-center h-10 \
               rounded-bl rounded-br shadow-md";

  view! {
    <div class=class >
      <span class="font-display font-bold text-xl text-product-11">
        "Rambit"
      </span>
      <div class="flex-1" />
      <a class="btn-link btn-link-secondary">Login</a>
      <a class="btn-link btn-link-primary">Sign Up</a>
    </div>
  }
}
