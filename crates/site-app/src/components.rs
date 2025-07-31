mod navbar {
  use leptos::prelude::*;

  #[component]
  pub fn Navbar() -> impl IntoView {
    let class = "bg-product-1 flex flex-row px-4 gap-2 items-center h-10 \
                 rounded-bl rounded-br shadow-md";

    view! {
      <div class=class >
        <span class="font-display font-bold text-xl text-product-11">
          "Rambit"
        </span>
      </div>
    }
  }
}

pub use self::navbar::*;
