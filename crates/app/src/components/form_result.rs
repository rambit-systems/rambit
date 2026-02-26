use maud::{Markup, html};

pub fn form_rejection(message: impl AsRef<str>) -> Markup {
  html! {
    p class="text-critical-11" {
      (message.as_ref())
    }
  }
}

pub fn form_acceptance(message: impl AsRef<str>) -> Markup {
  html! {
    p class="text-base-12" {
      (message.as_ref())
    }
  }
}

pub fn hint_neutral(message: impl AsRef<str>) -> Markup {
  html! {
    p class="text-sm" {
      (message.as_ref())
    }
  }
}

pub fn hint_warning(message: impl AsRef<str>) -> Markup {
  html! {
    p class="text-sm text-warn-11" {
      (message.as_ref())
    }
  }
}

pub fn hint_critical(message: impl AsRef<str>) -> Markup {
  html! {
    p class="text-sm text-critical-11" {
      (message.as_ref())
    }
  }
}
