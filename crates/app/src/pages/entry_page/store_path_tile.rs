use maud::{Markup, PreEscaped, html};
use models::StorePath;

use crate::components::icons::document_duplicate_hero_icon;

fn copy_button(text: &str) -> Markup {
  let onclick =
    PreEscaped(format!("navigator.clipboard.writeText('{}')", text));
  html! {
    button
      class="size-4 shrink-0 stroke-base-11 stroke-[1.5] hover:stroke-base-12 transition-colors"
      onclick=(onclick)
    {
      (document_duplicate_hero_icon())
    }
  }
}

pub fn store_path_tile(store_path: &StorePath<String>) -> Markup {
  let string = store_path.to_string();
  let separator_index =
    string.find('-').expect("no separator found in store path");
  let (digest, rest) = string.split_at(separator_index);
  let name = &rest[1..]; // skip the leading '-'

  const KEY_CLASS: &str = "place-self-end text-base-11";
  const VALUE_CLASS: &str = "text-base-12 font-medium font-mono";

  html! {
    div class="flex-1 p-6 elevation-flat flex flex-col gap-2" {
      p class="subtitle" { "Store Path Breakdown" }
      div class="flex-1 flex flex-col justify-around" {
        div class="grid gap-x-4 gap-y-2 grid-cols-[auto_1fr_auto] items-center" {
          p class=(KEY_CLASS) { "Prefix" }
          p class=(VALUE_CLASS) { "/nix/store/" }
          (copy_button("/nix/store/"))

          p class=(KEY_CLASS) { "Digest" }
          p class=(VALUE_CLASS) { (digest) }
          (copy_button(digest))

          p class=(KEY_CLASS) { "Name" }
          p class=(VALUE_CLASS) { (name) }
          (copy_button(name))
        }
      }
    }
  }
}
