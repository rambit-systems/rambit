//! R2 credentials sub-form fields for the create-store page.

use maud::{Markup, html};

use crate::components::icons::*;

pub(super) const ACCESS_KEY_FIELD_NAME: &str = "access_key";
pub(super) const SECRET_ACCESS_KEY_FIELD_NAME: &str = "secret_access_key";
pub(super) const BUCKET_FIELD_NAME: &str = "bucket";
pub(super) const ENDPOINT_FIELD_NAME: &str = "endpoint";

pub(super) fn credentials_input() -> Markup {
  let access_key_field = credentials_field(
    ACCESS_KEY_FIELD_NAME,
    "Access Key",
    "RGVjb2RlIHRoZSBzZWNyZXQga2V5",
    "text",
    html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (key_hero_icon()) } },
  );
  let secret_access_key_field = credentials_field(
    SECRET_ACCESS_KEY_FIELD_NAME,
    "Secret Access Key",
    "TXkgbXksIGFyZW4ndCB5b3UgcXVpdGUgYSBjdXJpb3VzIG9uZS4gYmFzZTY0QHJhbWJpdC5hcHA",
    "password",
    html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (key_hero_icon()) } },
  );
  let bucket_field = credentials_field(
    BUCKET_FIELD_NAME,
    "Bucket",
    "rambit-public-cache or similar",
    "text",
    html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (archive_box_hero_icon()) } },
  );
  let endpoint_field = credentials_field(
    ENDPOINT_FIELD_NAME,
    "Endpoint",
    "https://...",
    "text",
    html! { div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (globe_alt_hero_icon()) } },
  );

  html! {
    div class="flex flex-col gap-2" {
      (access_key_field)
      (secret_access_key_field)
      (bucket_field)
      (endpoint_field)
    }
  }
}

fn credentials_field(
  name: &str,
  label: &str,
  placeholder: &str,
  input_type: &str,
  icon: Markup,
) -> Markup {
  html! {
    label class="flex flex-col" {
      p class="text-sm" { (label) }
      div class="input-field" {
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
}
