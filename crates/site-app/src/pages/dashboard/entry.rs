use std::time::Duration;

use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::Entry;

use crate::{
  components::{
    refetch_while_focused, CacheItemLink, DataTableRefreshButton,
    StorePathAbbreviated, StorePathCopyButton, TableEmptyBody,
  },
  hooks::OrgHook,
  resources::entry::entries_in_org_query_scope,
};

#[island]
pub(super) fn EntryTable() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let key_fn = org_hook.key();
  let query_scope = entries_in_org_query_scope();

  refetch_while_focused(key_fn, query_scope.clone(), Duration::from_secs(10));

  let resource =
    expect_context::<QueryClient>().local_resource(query_scope.clone(), key_fn);

  let body_view = move |e: Vec<Entry>| {
    match e.len() {
      0 => view! {
        <EntryTableEmptyBody />
      }.into_any(),
      _ => view! {
        <tbody class="animate-fade-in min-h-10">
          <For each=move || e.clone() key=|e| e.id children=|e| view! { <EntryDataRow entry=e /> } />
        </tbody>
      }.into_any()
    }
  };
  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(entries) => body_view(entries).into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }
    })
  };

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Entries"</p>
      <div class="flex-1" />
      <DataTableRefreshButton
        key_fn=key_fn query_scope=query_scope.clone()
      />
    </div>

    <div class="w-full overflow-x-scroll">
      <table class="table">
        <thead>
          <th>"Store Path"</th>
          <th>"Caches"</th>
          <th>"File Size"</th>
          <th>"Ref Count"</th>
        </thead>
        <Transition fallback=|| ()>
          { suspend }
        </Transition>
      </table>
    </div>
  }
}

#[component]
fn EntryTableEmptyBody() -> impl IntoView {
  view! {
    <TableEmptyBody>
      <p class="text-base-12 text-lg">"Looks like you don't have any entries."</p>
      <p class="text-sm">"Upload some entries to see them here."</p>
    </TableEmptyBody>
  }
}

#[component]
fn EntryDataRow(entry: Entry) -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let base_url = org_hook.base_url();
  let entry_href = move || {
    format!("{base}/entry/{entry}", base = base_url(), entry = entry.id)
  };

  const ABBREVIATE_AFTER_COUNT: usize = 5;
  let mut caches = entry.caches.clone();
  caches.sort_unstable();
  let cache_count = caches.len();
  let mut caches = caches
    .into_iter()
    .take(ABBREVIATE_AFTER_COUNT)
    .map(|id| {
      view! {
        <CacheItemLink id=id />
      }
      .into_any()
    })
    .intersperse_with(|| ", ".into_any())
    .collect_view();
  if cache_count > ABBREVIATE_AFTER_COUNT {
    caches.push(", ...".into_any());
  }

  view! {
    <tr>
      <th scope="row" class="flex flex-row items-center gap-1">
        <a class="text-link text-link-primary" href=entry_href>
          <StorePathAbbreviated sp=entry.store_path.clone() />
        </a>
        <StorePathCopyButton sp=entry.store_path />
      </th>
      <td>
        { caches }
      </td>
      <td>{ entry.intrensic_data.nar_size.to_string() }</td>
      <td>{ entry.intrensic_data.references.len().to_string() }</td>
    </tr>
  }
}
