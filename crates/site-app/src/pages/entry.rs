use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, Entry};

use crate::{hooks::EntryHook, pages::UnauthorizedPage};

#[component]
pub fn EntryPage() -> impl IntoView {
  let params = use_params_map();
  let requested_entry = params()
    .get("entry")
    .expect("missing entry path param")
    .parse::<RecordId<_>>()
    .ok();

  requested_entry
    .map(|e| view! { <EntryTile id=e /> }.into_any())
    .unwrap_or(view! { <UnauthorizedPage /> }.into_any())
}

#[component]
fn EntryTile(id: RecordId<Entry>) -> impl IntoView {
  let entry_hook = EntryHook::new(move || id);

  view! {
    <div class="elevation-flat p-4 flex flex-col gap-4">
      <p class="title">"Entry"</p>
    </div>
  }
}
