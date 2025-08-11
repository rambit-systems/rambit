use leptos::prelude::*;
use models::{dvf::RecordId, Entry, Org};

use super::{DataTable, DataTableReloadButton};
use crate::components::CacheItemLink;

#[island]
pub(super) fn EntryTable(org: RecordId<Org>) -> impl IntoView {
  let key_fn = move || org;
  let fetcher = crate::resources::entry::fetch_entries_in_org;

  view! {
    <div class="flex flex-row items-start gap-2">
      <p class="title">"Entries"</p>
      <div class="flex-1" />
      <DataTableReloadButton
        key_fn=key_fn fetcher=fetcher
      />
    </div>

    <DataTable
      key_fn=key_fn fetcher=fetcher
      view_fn=move |e| view! { <EntryDataTable entries=e /> }
    />
  }
}

#[component]
fn EntryDataTable(entries: Vec<Entry>) -> impl IntoView {
  view! {
    <table class="table">
      <thead>
        <th>"Store Path"</th>
        <th>"Caches"</th>
        <th>"File Size"</th>
        <th>"Ref Count"</th>
      </thead>
      <tbody>
        { move || entries.iter().map(|e| view! {
          <EntryTableRow entry=e.clone() />
        }).collect::<Vec<_>>() }
      </tbody>
    </table>
  }
}

#[component]
fn EntryTableRow(entry: Entry) -> impl IntoView {
  view! {
    <tr>
      <th scope="row">
        <a class="text-link text-link-primary">
          { entry.store_path.to_string() }
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
