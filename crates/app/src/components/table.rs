use const_format::concatcp;
use maud::{Markup, html};

use crate::components::icons::loading_circle;

pub fn overlaid_table_body(children: Markup) -> Markup {
  const OUTER_CLASS: &str = "table-row animate-fade-in h-20 relative";
  const INNER_CLASS: &str = concatcp!(
    "absolute inset-0",
    " border-2 border-t-0 box-border border-base-6 border-dashed",
    " flex flex-col gap-0 justify-center items-center",
    " rounded-b"
  );

  html! {
    div class=(OUTER_CLASS) {
      div class=(INNER_CLASS) {
        (children)
      }
    }
  }
}

pub fn overlaid_message_table_body(title: Markup, subtitle: Markup) -> Markup {
  overlaid_table_body(html! {
    p class="text-base-12 text-lg" { (title) }
    @if !subtitle.0.is_empty() {
      p class="text-sm" { (subtitle) }
    }
  })
}

pub fn overlaid_critical_message_table_body(
  title: Markup,
  subtitle: Markup,
) -> Markup {
  overlaid_table_body(html! {
    p class="text-critical-11 text-lg" { (title) }
    @if !subtitle.0.is_empty() {
      p class="text-sm" { (subtitle) }
    }
  })
}

pub fn overlaid_loading_table_body() -> Markup {
  overlaid_message_table_body(
    html! {
      div class="flex flex-row items-center gap-2" {
        "Loading"
        div class="size-6 shrink-0" {
          (loading_circle())
        }
      }
    },
    html! {},
  )
}
