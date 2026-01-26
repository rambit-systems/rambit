mod copy_button;
mod create_button;
mod data_table;
mod footer;
pub mod form_layout;
mod icons;
mod input_field;
mod item_links;
mod navbar;
mod popover;
mod refetch_while_focused;
mod store_path;

use leptos::prelude::*;

pub use self::{
  copy_button::*, create_button::*, data_table::*, footer::*, icons::*,
  input_field::*, item_links::*, navbar::*, popover::*,
  refetch_while_focused::*, store_path::*,
};

pub fn form_rejection(text: impl AsRef<str>) -> impl IntoView {
  view! {
    <p id="form-rejection" class="text-critical-11">
      { text.as_ref() }
    </p>
  }
}

pub fn form_acceptance(text: impl AsRef<str>) -> impl IntoView {
  view! {
    <p id="form-acceptance" class="text-base-12">
      { text.as_ref() }
    </p>
  }
}
