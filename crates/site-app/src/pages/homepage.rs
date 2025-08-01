use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <div class="elevation-flat text-base-12">
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
  }
}
