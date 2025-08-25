use leptos::prelude::*;
use leptos_use::{use_clipboard, UseClipboardReturn};
use models::{Abbreviate, StorePath};

use crate::{components::HashtagHeroIcon, join_classes::JoinClasses};

#[component]
pub fn StorePathAbbreviated(sp: StorePath<String>) -> impl IntoView {
  Abbreviate::abbreviate(&sp)
}

#[island]
pub fn StorePathCopyButton(sp: StorePath<String>) -> impl IntoView {
  let absolute = sp.to_absolute_path();

  let UseClipboardReturn { copy, copied, .. } = use_clipboard();
  let copy = {
    let absolute = absolute.clone();
    move |_| copy(&absolute)
  };

  let copy_help_text = move || {
    if copied() {
      "Copied!"
    } else {
      "Click icon to copy"
    }
  };
  let tooltip_contents = Box::new(move || {
    view! {
      <div class="flex flex-col items-center gap-1">
        <p>{ absolute }</p>
        <p class="text-base-11">{ copy_help_text }</p>
      </div>
    }
    .into_any()
  });

  view! {
    <Tooltip content=tooltip_contents>
      <div
        on:click=copy
        class="p-1 rounded hover:bg-product-4 active:bg-active-5 pointer-cursor transition-colors"
      >
        <HashtagHeroIcon {..} class="size-4 stroke-base-11 stroke-[2.0]" />
      </div>
    </Tooltip>
  }
}

#[component]
fn Tooltip(children: Children, content: Children) -> impl IntoView {
  let tooltip_class = [
    "absolute left-1/2 -translate-x-1/2 bottom-full mb-2 w-max",
    "elevation-lv2 border-[1.5px] border-base-6",
    "text-sm text-base-12 font-normal",
    "px-4 py-2",
    "opacity-0 pointer-events-none transition ease-out",
    "group-hover:opacity-100 group-focus:opacity-100",
  ]
  .join_classes();
  const TOOLTIP_ARROW_CLASS: &str = "absolute left-1/2 -translate-x-1/2 \
                                     -bottom-1 w-2 h-2 bg-base-1
rotate-45";

  view! {
    <span class="relative group">
      <div class=tooltip_class>
        { content() }
        <div class=TOOLTIP_ARROW_CLASS></div>
      </div>
      { children() }
    </span>
  }
}
