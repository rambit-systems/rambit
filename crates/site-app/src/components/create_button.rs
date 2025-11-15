use leptos::prelude::*;

use crate::hooks::OrgHook;

#[component]
pub fn CreateCacheButton(text: &'static str) -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let href = org_hook.create_cache_url();

  view! {
    <a href=href class="btn btn-primary-subtle">
      { text }
    </a>
  }
}

#[component]
pub fn CreateStoreButton(text: &'static str) -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let href = org_hook.create_store_url();

  view! {
    <a href=href class="btn btn-primary-subtle">
      { text }
    </a>
  }
}
