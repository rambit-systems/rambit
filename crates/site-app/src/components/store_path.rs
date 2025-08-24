use leptos::prelude::*;
use models::{Abbreviate, StorePath};

use crate::join_classes::JoinClasses;

#[component]
pub fn StorePath(sp: StorePath<String>) -> impl IntoView {
  let full = sp.to_string();
  let abbreviated = Abbreviate::abbreviate(&sp);

  view! {
    <Tooltip content=full>
      { abbreviated }
    </Tooltip>
  }
}

#[component]
fn Tooltip(children: Children, #[prop(into)] content: String) -> impl IntoView {
  let tooltip_class = [
    "absolute left-1/2 -translate-x-1/2 bottom-full mb-2 w-max",
    "elevation-lv2",
    "text-sm text-base-12 font-normal",
    "px-3 py-1",
    "opacity-0 pointer-events-none transition ease-out",
    "group-hover:opacity-100 group-focus:opacity-100",
  ]
  .join_classes();
  const TOOLTIP_ARROW_CLASS: &str =
    "absolute left-1/2 -translate-x-1/2 -bottom-1 w-2 h-2 bg-base-1 rotate-45";

  view! {
    <span class="relative group">
      <div class=tooltip_class>
        { content }
        <div class=TOOLTIP_ARROW_CLASS></div>
      </div>
      { children() }
    </span>
  }
}
