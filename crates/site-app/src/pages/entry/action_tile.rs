use leptos::prelude::*;

#[component]
pub(crate) fn ActionTile() -> impl IntoView {
  view! {
    <div class="md:w-64 lg:w-80 p-6 elevation-flat flex flex-col gap-4 align-self-start">
      <p class="subtitle">"Actions"</p>
      <div class="flex flex-col gap-2">
        <button class="btn btn-critical">
          "Delete Entry"
        </button>
      </div>
    </div>
  }
}
