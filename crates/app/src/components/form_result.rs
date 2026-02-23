use maud::{Markup, html};

pub fn form_rejection(message: impl AsRef<str>) -> Markup {
  html! {
    p id="form-rejection" class="text-critical-11" {
      (message.as_ref())
    }
  }
}

pub fn form_acceptance(message: impl AsRef<str>) -> Markup {
  html! {
    p id="form-rejection" class="text-base-12" {
      (message.as_ref())
    }
  }
}
