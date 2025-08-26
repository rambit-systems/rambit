use std::time::Duration;

use leptos::prelude::*;
use models::Entry;

use crate::{
  components::{
    refetch_while_focused, CacheItemLink, DataTable, DataTableRefreshButton,
    StorePathAbbreviated, StorePathCopyButton,
  },
  hooks::OrgHook,
  resources::entry::entries_in_org_query_scope,
};

#[island]
pub(super) fn EntryTable() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let key_fn = org_hook.key();
  let query_scope = entries_in_org_query_scope();

  refetch_while_focused(key_fn, query_scope.clone(), Duration::from_secs(30));

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Entries"</p>
      <div class="flex-1" />
      <DataTableRefreshButton
        key_fn=key_fn query_scope=query_scope.clone()
      />
    </div>

    <table class="table">
      <thead>
        <th>"Store Path"</th>
        <th>"Caches"</th>
        <th>"File Size"</th>
        <th>"Ref Count"</th>
      </thead>
      <DataTable
        key_fn=key_fn query_scope=query_scope
        view_fn=move |e| view! {
          <tbody class="min-h-10 animate-fade-in">
            <For each=e key=|e| e.id children=|e| view! { <EntryDataRow entry=e /> } />
          </tbody>
        }
      />
    </table>
  }
}

#[component]
fn EntryDataRow(entry: Entry) -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let entry_href = move || {
    format!(
      "/org/{org}/entry/{entry}",
      org = org_hook.key()(),
      entry = entry.id
    )
  };

  view! {
    <tr>
      <th scope="row" class="flex flex-row items-center gap-1">
        <a class="text-link text-link-primary" href=entry_href>
          <StorePathAbbreviated sp=entry.store_path.clone() />
        </a>
        <StorePathCopyButton sp=entry.store_path />
      </th>
      <td>
        { entry.caches.clone().into_iter().map(|id| view! {
          <CacheItemLink id=id />
        }).collect_view()}
      </td>
      <td>{ entry.intrensic_data.nar_size.to_string() }</td>
      <td>{ entry.intrensic_data.references.len().to_string() }</td>
    </tr>
  }
}
