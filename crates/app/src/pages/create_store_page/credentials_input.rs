//! R2 credentials sub-form fields for the create-store page.

use maud::{Markup, html};

use crate::components::icons::*;

pub(super) const ACCESS_KEY_FIELD_NAME: &str = "access_key";
pub(super) const SECRET_ACCESS_KEY_FIELD_NAME: &str = "secret_access_key";
pub(super) const BUCKET_FIELD_NAME: &str = "bucket";
pub(super) const ENDPOINT_FIELD_NAME: &str = "endpoint";

pub(super) fn credentials_input() -> Markup {
  html! {
    div class="flex flex-col gap-2" {
      (credentials_field(
        ACCESS_KEY_FIELD_NAME,
        "Access Key",
        "text",
        html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (key_hero_icon()) } },
      ))
      (credentials_field(
        SECRET_ACCESS_KEY_FIELD_NAME,
        "Secret Access Key",
        "password",
        html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (key_hero_icon()) } },
      ))
      (credentials_field(
        BUCKET_FIELD_NAME,
        "Bucket",
        "text",
        html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (archive_box_hero_icon()) } },
      ))
      (credentials_field(
        ENDPOINT_FIELD_NAME,
        "Endpoint",
        "text",
        html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (globe_alt_hero_icon()) } },
      ))
    }
  }
}

fn credentials_field(
  name: &str,
  placeholder: &str,
  input_type: &str,
  icon: Markup,
) -> Markup {
  html! {
    label class="input-field" {
      (icon)
      input
        class="w-full py-2 focus-visible:outline-none"
        type=(input_type)
        placeholder=(placeholder)
        name=(name)
        required;
    }
  }
}
