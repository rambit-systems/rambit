use maud::{Markup, html};

pub fn grid_row(children: Markup) -> Markup {
  html! {
    div class="flex flex-col gap-2 md:contents" {
      (children)
    }
  }
}

pub fn grid_row_full(children: Markup) -> Markup {
  html! {
    div class="flex flex-col gap-2 md:col-span-2" {
      (children)
    }
  }
}

pub fn grid_row_label(title: impl AsRef<str>, desc: impl AsRef<str>) -> Markup {
  html! {
    div class="flex flex-col gap-0.5" {
      p class="text-base-12" { (title.as_ref()) }
      p class="text-sm max-w-prose" { (desc.as_ref()) }
    }
  }
}
