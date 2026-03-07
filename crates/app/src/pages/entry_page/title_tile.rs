use maud::{Markup, PreEscaped, html};
use models::StorePath;

use crate::components::icons::document_duplicate_hero_icon;

pub fn title_tile(store_path: &StorePath<String>) -> Markup {
  let path = store_path.to_absolute_path();
  let onclick = PreEscaped(format!(
    "navigator.clipboard.writeText('{}')",
    path
  ));

  html! {
    div class="p-6 elevation-flat flex flex-row gap-2 items-center" {
      p class="text-base-12 text-xl" {
        span class="font-bold" { "Entry: " }
        (path)
      }
      button
        class="size-5 shrink-0 stroke-base-11 stroke-[1.5] hover:stroke-base-12 transition-colors"
        onclick=(onclick)
      {
        (document_duplicate_hero_icon())
      }
    }
  }
}
