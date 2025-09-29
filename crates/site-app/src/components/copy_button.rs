use leptos::prelude::*;

use crate::components::DocumentDuplicateHeroIcon;

#[island]
pub fn CopyButton(copy_content: String) -> impl IntoView {
  let _copy_content = Signal::stored(copy_content);

  let on_click = move |_| {
    #[cfg(feature = "hydrate")]
    (leptos_use::use_clipboard().copy)(&_copy_content())
  };

  const CLASS: &str = "cursor-pointer stroke-[2.0] stroke-base-11/50 \
                       hover:stroke-base-11/75 transition-colors";

  view! {
    <DocumentDuplicateHeroIcon on:click={on_click} {..} class=CLASS />
  }
}
