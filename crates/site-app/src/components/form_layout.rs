use leptos::prelude::*;

#[component]
pub fn GridRow(children: Children) -> impl IntoView {
  view! {
    <div class="flex flex-col gap-2 md:contents">
      { children() }
    </div>
  }
}

#[component]
pub fn GridRowFull(children: Children) -> impl IntoView {
  view! {
    <div class="flex flex-col gap-2 md:col-span-2">
      { children() }
    </div>
  }
}

#[component]
pub fn GridRowLabel(
  #[prop(into)] title: String,
  #[prop(into)] desc: String,
) -> impl IntoView {
  view! {
    <div class="flex flex-col gap-0.5">
      <p class="text-base-12">{ title }</p>
      <p class="text-sm max-w-prose">{ desc }</p>
    </div>
  }
}
