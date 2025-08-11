use leptos::prelude::*;
use models::{dvf::RecordId, Entry, Org};

use crate::components::CacheItemLink;

#[component]
pub(super) fn EntryDashboardTile(org: RecordId<Org>) -> impl IntoView {
  let entries_in_org = crate::resources::entry::entries_in_org(move || org);
  let suspend = move || {
    Suspend::new(async move {
      match entries_in_org.await {
        Ok(entries) => view! { <EntryTable entries=entries /> }.into_any(),
        Err(e) => view! { { format!("Error: {e}") } }.into_any(),
      }
    })
  };

  view! {
    <div class="col-span-2 p-6 elevation-flat flex flex-col gap-4">
      <p class="title">"Entries"</p>

      <Suspense fallback=move || view! { "Loading..." }>
        { suspend }
      </Suspense>
    </div>
  }
}

#[component]
fn EntryTable(entries: Vec<Entry>) -> impl IntoView {
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
      <th scope="row">{ entry.store_path.to_string() }</th>
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
