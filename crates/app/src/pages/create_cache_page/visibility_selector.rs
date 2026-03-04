//! Visibility radio-group component for the create-cache form.

use maud::{Markup, html};

use super::VISIBILITY_FIELD_NAME;

pub(super) fn visibility_selector() -> Markup {
  html! {
    fieldset class="flex flex-col gap-4" {
      (visibility_option("Private", "private", true, html! {
        "Your entries are only available to users in your organization."
      }))
      (visibility_option("Public", "public", false, html! {
        "Your entries are available for everyone to use."
      }))
    }
  }
}

fn visibility_option(
  name: &str,
  value: &str,
  default: bool,
  description: Markup,
) -> Markup {
  const OUTER_CLASS: &str =
    "group flex-1 flex flex-col gap-2 px-4 py-3 max-w-80 hover:elevation-lv1 \
     cursor-pointer transition rounded border-2 border-base-7 \
     hover:border-base-8 bg-gradient-to-tr from-transparent to-transparent \
     to-50% has-checked:border-product-7 has-checked:hover:border-product-8 \
     has-checked:from-product-3";
  const TITLE_CLASS: &str = "text-base-12 text-lg font-semibold leading-none";
  const DESCRIPTION_CLASS: &str = "text-sm leading-[1.1]";
  const BUTTON_BACKGROUND_CLASS: &str = "size-5 bg-base-2 border \
                                         border-base-6 rounded-full flex \
                                         flex-row justify-center items-center";
  const BUTTON_CLASS: &str = "size-3 bg-product-9 rounded-full \
                              transition-opacity opacity-0 \
                              group-has-checked:opacity-100";

  html! {
    label class=(OUTER_CLASS) {
      input
        type="radio"
        name=(VISIBILITY_FIELD_NAME)
        value=(value)
        class="hidden"
        required
        checked[default];
      div class="flex flex-row justify-between items-center" {
        p class=(TITLE_CLASS) { (name) }
        div class=(BUTTON_BACKGROUND_CLASS) {
          div class=(BUTTON_CLASS) {}
        }
      }
      div class=(DESCRIPTION_CLASS) { (description) }
    }
  }
}
