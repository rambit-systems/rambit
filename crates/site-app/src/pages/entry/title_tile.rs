use leptos::prelude::*;
use models::StorePath;

use crate::components::CopyButton;

#[component]
pub(crate) fn TitleTile(store_path: StorePath<String>) -> impl IntoView {
  let path = store_path.to_absolute_path();
  view! {
    <div class="p-6 elevation-flat flex flex-row gap-2 items-center">
      <p class="text-base-12 text-xl">
        <span class="font-bold">
          "Entry: "
        </span>
        { path.clone() }
      </p>
      <CopyButton copy_content={path} {..} class="size-5" />
    </div>
  }
}
