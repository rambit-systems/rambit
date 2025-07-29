use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
  view! {
    <div class="container mx-auto flex flex-col gap-8 min-h-screen pb-8">
      <Navbar />

      <div class="bg-white flex-1 border rounded">
        <div class="p-24 font-display">
          <p class="text-3xl">
            "Welcome to"
          </p>
          <p class="text-8xl">
            "Rambit"
          </p>
        </div>
      </div>
    </div>
  }
}

#[component]
fn Navbar() -> impl IntoView {
  let class = "bg-white flex flex-row px-4 gap-2 items-center h-10 rounded-bl \
               rounded-br shadow";

  view! {
    <div class=class >
      <span class="font-display text-xl font-bold">
        "Rambit"
      </span>
    </div>
  }
}
