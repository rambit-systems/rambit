use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <div class="mx-4 sm:container sm:mx-auto flex flex-col gap-8 min-h-screen pb-8">
      <Navbar />

      <div class="bg-base-1 border border-base-6 rounded">
        <div class="p-6 sm:p-20 font-semibold flex flex-col gap-2">
          <p class="text-sm sm:text-lg text-product-11 uppercase">
            "Welcome to Rambit Labs"
          </p>
          <div class="text-4xl sm:text-6xl font-display tracking-tight">
            <p class="font-thin">
              "Integrate and"
            </p>
            <p>"never waste"</p>
            <p>
              "another "
              <span class="text-product-11">"build"</span>
            </p>
          </div>
        </div>
      </div>
    </div>
  }
}

#[component]
fn Navbar() -> impl IntoView {
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
