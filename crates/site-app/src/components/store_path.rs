use leptos::prelude::*;
use models::nix_compat::store_path::ENCODED_DIGEST_SIZE;

#[component]
pub fn StorePath(sp: models::StorePath<String>) -> impl IntoView {
  const COUNT: usize = 4;
  let string = sp.to_string();
  let separator_index =
    string.find('-').expect("no separator found in store path");
  let (digest, rest) = string.split_at(separator_index);
  let display = format!(
    "{first}…{last}{rest}",
    first = &digest[0..COUNT],
    last = &digest[ENCODED_DIGEST_SIZE - COUNT..ENCODED_DIGEST_SIZE]
  );

  view! {
    <a class="text-link">
      { display }
    </a>
  }
}
