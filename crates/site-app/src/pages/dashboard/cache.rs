use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::Visibility, PvCache};

use crate::{
  components::{
    CacheItemLink, CreateCacheButton, DataTableRefreshButton,
    LockClosedHeroIcon,
  },
  hooks::OrgHook,
  resources::cache::caches_in_org_query_scope,
};

#[island]
pub(super) fn CacheTable() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let key_fn = org_hook.key();
  let query_scope = caches_in_org_query_scope();

  let resource =
    expect_context::<QueryClient>().local_resource(query_scope.clone(), key_fn);

  let body_view = move |caches: Vec<PvCache>| {
    view! {
      <tbody class="animate-fade-in min-h-10">
        <For each=move || caches.clone() key=|r| r.id children=|r| view! { <CacheDataRow cache=r /> } />
      </tbody>
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

    <div class="w-full overflow-x-scroll">
      <table class="table">
        <thead>
          <th>"Name"</th>
          <th>"Visibility"</th>
        </thead>
        <Transition fallback=|| ()>
          { suspend }
        </Transition>
      </table>
    </div>
  }
}

#[component]
fn CacheDataRow(cache: PvCache) -> impl IntoView {
  view! {
    <tr>
      <th scope="row"><CacheItemLink id=cache.id extra_class="text-link-primary"/></th>
      <td class="flex flex-row items-center gap-1">
        { cache.visibility.to_string() }
        { matches!(cache.visibility, Visibility::Private).then_some(view! {
          <LockClosedHeroIcon {..} class="size-4 stroke-base-11/75 stroke-[2.0]" />
        })}
      </td>
    </tr>
  }
}
