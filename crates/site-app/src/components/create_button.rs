use leptos::prelude::*;

use crate::hooks::OrgHook;

#[component]
pub fn CreateCacheButton(text: &'static str) -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let base_url = org_hook.base_url();
  let href =
    Signal::derive(move || format!("{base}/create_cache", base = base_url()));

  view! {
    <a href=href class="btn btn-primary-subtle">
      { text }
    </a>
  }
}
