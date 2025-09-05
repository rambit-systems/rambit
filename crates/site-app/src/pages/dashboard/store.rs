use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{PvStorageCredentials, PvStore};

use crate::{
  components::{DataTableRefreshButton, StoreItemLink},
  hooks::OrgHook,
  resources::store::{
    entry_count_in_store_query_scope, stores_in_org_query_scope,
  },
};

#[island]
pub(super) fn StoreTable() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let key_fn = org_hook.key();
  let query_scope = stores_in_org_query_scope();
  let resource =
    expect_context::<QueryClient>().local_resource(query_scope.clone(), key_fn);

  let body_view = move |stores: Vec<PvStore>| {
    view! {
      <tbody class="min-h-10">
        <For each=move || stores.clone() key=|r| r.id children=|r| view! { <StoreDataRow store=r /> } />
      </tbody>
    }
  };
  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(stores) => body_view(stores).into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }
    })
  };

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
      <Transition fallback=|| ()>
        { suspend }
      </Transition>
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
      <td><Transition fallback=|| "[loading]">
        { entry_count_suspend }
      </Transition></td>
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
