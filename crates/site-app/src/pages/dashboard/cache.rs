use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{PvCache, Visibility};

use crate::{
  components::{
    CacheItemLink, CreateCacheButton, DataTableRefreshButton, LoadingCircle,
    LockClosedHeroIcon, TableEmptyBody,
  },
  formatting_utils::ThousandsSeparated,
  hooks::OrgHook,
  resources::cache::{
    caches_in_org_query_scope, entry_count_in_cache_query_scope,
  },
};

#[component]
pub(super) fn CacheTable() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let table_infill_url = org_hook.dashboard_cache_table_infill_url();

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Caches"</p>
      <div class="flex-1" />

      <button
        class="btn btn-secondary relative overflow-hidden"
        hx-get={ table_infill_url }
        hx-target="#cache-table"
      >
        "Refresh"
        <div class="absolute inset-0 flex flex-row justify-center items-center btn-secondary htmx-indicator">
          <LoadingCircle {..} class="size-5" />
        </div>
      </button>
      <CreateCacheButton text="Create..." />
    </div>

    <div class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <th>"Name"</th>
          <th>"Visibility"</th>
          <th>"Entry Count"</th>
        </thead>
        <div id="cache-table" class="contents">
          <CacheTableInfill />
        </div>
      </table>
    </div>
  }
}

#[component]
pub(super) fn CacheTableInfill() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let key_fn = org_hook.key();
  let query_scope = caches_in_org_query_scope();

  let resource =
    expect_context::<QueryClient>().resource(query_scope.clone(), key_fn);

  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(caches) => view! {
          <CacheTableBodyData caches=caches />
        }
        .into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }
    })
  };

  view! {
    <Suspense fallback=|| "[loading]">
      { suspend }
    </Suspense>
  }
}

#[component]
pub(super) fn CacheTableBodyData(caches: Vec<PvCache>) -> impl IntoView {
  view! {
    <tbody class="animate-fade-in min-h-10">
      <For each=move || caches.clone() key=|r| r.id children=|r| view! { <CacheDataRow cache=r /> } />
    </tbody>
  }
}

#[component]
fn CacheTableEmptyBody() -> impl IntoView {
  // let org_hook = OrgHook::new_requested();
  // let create_url = org_hook.create_cache_url();

  // view! {
  //   <TableEmptyBody>
  //     <p class="text-base-12 text-lg">"Looks like you don't have any
  // caches."</p>     <p class="text-sm">
  //       <a href=create_url class="text-link text-link-primary">"Create
  // one"</a>       " to get started."
  //     </p>
  //   </TableEmptyBody>
  // }
}

#[component]
fn CacheDataRow(cache: PvCache) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let entry_count_query_scope = entry_count_in_cache_query_scope();
  let entry_count_key = move || cache.id;
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

  view! {
    <tr>
      <th scope="row"><CacheItemLink id=cache.id extra_class="text-link-primary"/></th>
      <td class="flex flex-row items-center gap-1">
        { cache.visibility.to_string() }
        { matches!(cache.visibility, Visibility::Private).then_some(view! {
          <LockClosedHeroIcon {..} class="size-4 stroke-base-11/75 stroke-[2.0]" />
        })}
      </td>
      <td><Transition fallback=|| "[loading]">
        { entry_count_suspend }
      </Transition></td>
    </tr>
  }
}
