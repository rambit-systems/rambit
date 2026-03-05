use maud::{Markup, html};

pub fn loading() -> Markup {
  html! {
    span class="text-base-11" {
      "[loading]"
    }
  }
}

pub fn unauthorized() -> Markup {
  html! {
    span class="text-base-11" {
      "[unauthorized]"
    }
  }
}

pub fn missing() -> Markup {
  html! {
    span class="text-base-11" {
      "[missing]"
    }
  }
}

pub fn error() -> Markup {
  html! {
    span class="text-base-11" {
      "[error]"
    }
  }
}
