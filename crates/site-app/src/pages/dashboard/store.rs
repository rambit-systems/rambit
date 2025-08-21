use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Org, PvStorageCredentials, PvStore};

use crate::{
  components::{DataTable, DataTableRefreshButton, StoreItemLink},
  resources::store::{
    entry_count_in_store_query_scope, stores_in_org_query_scope,
  },
};

#[island]
pub(super) fn StoreTable(org: RecordId<Org>) -> impl IntoView {
  let key_fn = move || org;
  let query_scope = stores_in_org_query_scope();

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Stores"</p>
      <div class="flex-1" />
      <DataTableRefreshButton
        key_fn=key_fn query_scope=query_scope.clone()
      />
    </div>

    <table class="table">
      <thead>
        <th>"Name"</th>
        <th>"Entry Count"</th>
        <th>"Storage Type"</th>
      </thead>
      <DataTable
        key_fn=key_fn query_scope=query_scope
        view_fn=move |c| view! {
          <tbody class="min-h-10">
            <For each=c key=|c| c.id children=|c| view! { <StoreDataRow store=c /> } />
          </tbody>
        }
      />
    </table>
  }
}

#[component]
fn StoreDataRow(store: PvStore) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let entry_count_query_scope = entry_count_in_store_query_scope();
  let entry_count_key = move || store.id;
  let entry_count_resource =
    query_client.resource(entry_count_query_scope, entry_count_key);
  let entry_count_suspend = move || {
    Suspend::new(async move {
      match entry_count_resource.await {
        Ok(count) => format_with_commas(count).into_any(),
        Err(_) => "[error]".into_any(),
      }
    })
  };

  let storage_type = match store.credentials {
    PvStorageCredentials::Local(_) => view! { "Local (DEBUG)" }.into_any(),
    PvStorageCredentials::R2(_) => view! { "R2" }.into_any(),
    PvStorageCredentials::Memory(_) => view! { "Memory (DEBUG)" }.into_any(),
  };

  view! {
    <tr>
      <th scope="row"><StoreItemLink id=store.id extra_class="text-link-primary"/></th>
      <td><Suspense fallback=|| "[loading]">
        { entry_count_suspend }
      </Suspense></td>
      <td>{ storage_type }</td>
    </tr>
  }
}

fn format_with_commas(n: u32) -> String {
  let s = n.to_string();
  let chars: Vec<char> = s.chars().collect();
  let mut result = String::new();

  for (i, ch) in chars.iter().enumerate() {
    if i > 0 && (chars.len() - i).is_multiple_of(3) {
      result.push(',');
    }
    result.push(*ch);
  }

  result
}
