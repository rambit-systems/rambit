use leptos::prelude::*;
use models::StorePath;

use crate::components::CopyButton;

#[component]
pub(crate) fn StorePathTile(store_path: StorePath<String>) -> impl IntoView {
  let string = store_path.to_string();
  let separator_index =
    string.find('-').expect("no separator found in store path");
  let (digest, _) = string.split_at(separator_index);
  let name = store_path.name().clone();

  const KEY_CLASS: &str = "place-self-end";
  const VALUE_CLASS: &str = "text-base-12 font-medium";

  let value_element = move |s: &str| {
    view! {
      <div class="flex flex-row gap-2 items-center">
        <p class=VALUE_CLASS>{ s }</p>
        <CopyButton
          copy_content={s.to_string()}
          {..} class="size-4"
        />
      </div>
    }
  };

  view! {
    <div class="p-6 elevation-flat flex flex-col gap-2">
      <p class="subtitle">
        "Store Path Breakdown"
      </p>
      <div class="flex-1 flex flex-col justify-around">
        <div class="grid gap-x-4 gap-y-1 grid-cols-[repeat(2,auto)]">
          <p class=KEY_CLASS>"Prefix"</p>
          { value_element("/nix/store/") }
          <p class=KEY_CLASS>"Digest"</p>
          { value_element(digest) }
          <p class=KEY_CLASS>"Name"</p>
          { value_element(&name) }
        </div>
      </div>
    </div>
  }
}
