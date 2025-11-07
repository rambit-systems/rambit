use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{PvR2StorageCredentials, PvStorageCredentials, PvStore};

use crate::{
  components::{
    CreateStoreButton, DataTableRefreshButton, StoreItemLink, TableEmptyBody,
  },
  formatting_utils::ThousandsSeparated,
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
    match stores.len() {
    0 => view! {
      <StoreTableEmptyBody />
    }.into_any(),
    _ => view! {
      <tbody class="animate-fade-in min-h-10">
        <For each=move || stores.clone() key=|r| r.id children=|r| view! { <StoreDataRow store=r /> } />
      </tbody>
    }.into_any()
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
      <CreateStoreButton text="Create..." />
    </div>

    <div class="w-full overflow-x-auto">
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
    </div>
  }
}

#[component]
fn StoreTableEmptyBody() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let create_url =
    Signal::derive(move || format!("{}/create_store", org_hook.base_url()()));

  view! {
    <TableEmptyBody>
      <p class="text-base-12 text-lg">"Looks like you don't have any stores."</p>
      <p class="text-sm">
        <a href=create_url class="text-link text-link-primary">"Create one"</a>
        " to get started."
      </p>
    </TableEmptyBody>
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
        Ok(count) => ThousandsSeparated(count).to_string().into_any(),
        Err(_) => "[error]".into_any(),
      }
    })
  };

  let storage_type = match store.credentials {
    PvStorageCredentials::R2(PvR2StorageCredentials::Default {
      bucket,
      ..
    }) => format!("R2 ({bucket})").into_any(),
    PvStorageCredentials::Memory(_) => "Memory (DEBUG)".into_any(),
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
