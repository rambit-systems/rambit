use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{PvCache, Visibility};

use crate::{
  components::{
    CacheItemLink, CreateCacheButton, DataTableRefreshButton,
    LockClosedHeroIcon, TableEmptyBody,
  },
  formatting_utils::ThousandsSeparated,
  hooks::OrgHook,
  resources::cache::{
    caches_in_org_query_scope, entry_count_in_cache_query_scope,
  },
};

#[island]
pub(super) fn CacheTable() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let key_fn = org_hook.key();
  let query_scope = caches_in_org_query_scope();

  let resource =
    expect_context::<QueryClient>().local_resource(query_scope.clone(), key_fn);

  let body_view = move |caches: Vec<PvCache>| {
    match caches.len() {
    0 => view! {
      <CacheTableEmptyBody />
    }.into_any(),
    _ => view! {
      <tbody class="animate-fade-in min-h-10">
        <For each=move || caches.clone() key=|r| r.id children=|r| view! { <CacheDataRow cache=r /> } />
      </tbody>
    }.into_any()
  }
  };
  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(caches) => body_view(caches).into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }
    })
  };

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Caches"</p>
      <div class="flex-1" />
      <DataTableRefreshButton
        key_fn=key_fn query_scope=query_scope.clone()
      />
      <CreateCacheButton text="Create..." />
    </div>

    <div class="w-full overflow-x-auto">
      <table class="table">
        <thead>
          <th>"Name"</th>
          <th>"Visibility"</th>
          <th>"Entry Count"</th>
        </thead>
        <Transition fallback=|| ()>
          { suspend }
        </Transition>
      </table>
    </div>
  }
}

#[component]
fn CacheTableEmptyBody() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let create_url =
    Signal::derive(move || format!("{}/create_cache", org_hook.base_url()()));

  view! {
    <TableEmptyBody>
      <p class="text-base-12 text-lg">"Looks like you don't have any caches."</p>
      <p class="text-sm">
        <a href=create_url class="text-link text-link-primary">"Create one"</a>
        " to get started."
      </p>
    </TableEmptyBody>
  }
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
