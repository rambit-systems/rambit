use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::Entry;

use crate::{
  components::{
    CacheItemLink, LoadingCircle, StorePathAbbreviated, StorePathCopyButton,
    TableEmptyBody,
  },
  hooks::OrgHook,
  resources::entry::entries_in_org_query_scope,
};

#[component]
pub(super) fn EntryTable() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let table_infill_url = org_hook.dashboard_entry_table_infill_url();

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Entries"</p>
      <div class="flex-1" />

      <button
        class="btn btn-secondary relative overflow-hidden"
        hx-get={ table_infill_url }
        hx-target="#entry-table"
      >
        "Refresh"
        <div class="absolute inset-0 flex flex-row justify-center items-center btn-secondary htmx-indicator">
          <LoadingCircle {..} class="size-5" />
        </div>
      </button>
    </div>

    <div class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <th>"Store Path"</th>
          <th>"Caches"</th>
          <th>"File Size"</th>
          <th>"Ref Count"</th>
        </thead>
        <div id="entry-table" class="contents">
          <EntryTableInfill />
        </div>
      </table>
    </div>
  }
}

#[component]
pub(super) fn EntryTableInfill() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let key_fn = org_hook.key();
  let query_scope = entries_in_org_query_scope();

  let resource =
    expect_context::<QueryClient>().resource(query_scope.clone(), key_fn);

  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(entries) => view! {
          <EntryTableBody entries=entries />
        }
        .into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }
    })
  };

  view! {
    <tbody class="animate-fade-in min-h-10 relative">
      <Transition fallback=|| "[loading]">
        { suspend }
      </Transition>
    </tbody>
  }
}

#[component]
fn EntryTableEmptyBody() -> impl IntoView {
  const INNER_CLASS: &str = "absolute inset-0 flex flex-col items-center \
                             justify-center border-[2px] box-border \
                             border-t-0 border-base-6 border-dashed rounded-b";

  view! {
    <tr> <td/><td/><td/><td/> </tr>
    <tr> <td/><td/><td/><td/> </tr>
    <div class=INNER_CLASS>
      <p class="text-base-12 text-lg">"Looks like you don't have any entries."</p>
      <p class="text-sm">"Upload some entries from the CLI to see them here."</p>
    </div>
  }
}

#[component]
fn EntryTableBody(entries: Vec<Entry>) -> impl IntoView {
  // if entries.is_empty() {
  //   return view! { <EntryTableEmptyBody /> }.into_any();
  // };

  view! {
    <For each=move || entries.clone() key=|e| e.id children=|e| view! { <EntryDataRow entry=e /> } />
  }.into_any()
}

#[component]
fn EntryDataRow(entry: Entry) -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let entry_href = org_hook.entry_url(entry.id);

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
