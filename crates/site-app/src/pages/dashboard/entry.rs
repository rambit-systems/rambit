use leptos::prelude::*;
use models::{dvf::RecordId, Entry, Org};

use crate::{
  components::{CacheItemLink, DataTable, DataTableRefreshButton, StorePath},
  resources::entry::entries_in_org_query_scope,
};

#[island]
pub(super) fn EntryTable(org: RecordId<Org>) -> impl IntoView {
  let key_fn = move || org;
  let query_scope = entries_in_org_query_scope();

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
          <tbody class="min-h-10">
            <For each=e key=|e| e.id children=|e| view! { <EntryDataRow entry=e /> } />
          </tbody>
        }
      />
    </table>
  }
}

#[component]
fn EntryDataRow(entry: Entry) -> impl IntoView {
  view! {
    <tr>
      <th scope="row">
        <a class="text-link text-link-primary">
          <StorePath sp=entry.store_path />
        </a>
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
