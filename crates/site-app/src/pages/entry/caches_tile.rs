use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{Cache, Entry, PvCache, RecordId, StorePath, Visibility};

use crate::components::{CacheItemLink, LockClosedHeroIcon};

#[component]
pub(crate) fn CachesTile(entry: Entry) -> impl IntoView {
  let caches = Signal::stored(entry.caches.clone());
  let store_path = Signal::stored(entry.store_path);
  view! {
    <div class="flex-1 p-6 elevation-flat flex flex-col gap-4">
      <div class="flex flex-col gap-1">
        <p class="subtitle">
          "Resident Caches"
        </p>
        <p class="max-w-prose">
          "These are the caches that the entry is attached to."
        </p>
      </div>

      <table class="table">
        <thead>
          <th>"Name"</th>
          <th>"Download Url"</th>
          <th>"Visibility"</th>
        </thead>
        <tbody class="animate-fade-in min-h-10">
          <For each=caches key=|r| *r children=move |r| view! {
            <CachesTileRow store_path=store_path cache_id=r />
          } />
        </tbody>
      </table>
    </div>
  }
}

#[component]
pub(crate) fn CachesTileRow(
  store_path: Signal<StorePath<String>>,
  cache_id: RecordId<Cache>,
) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let query_scope = crate::resources::cache::cache_query_scope();
  let resource = query_client.resource(query_scope, move || cache_id);

  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(Some(c)) => {
          view! { <CachesTileDataRow cache=c store_path=store_path /> }
            .into_any()
        }
        Ok(None) => None::<()>.into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }
    })
  };

  view! {
    <Transition fallback=|| ()>{ suspend }</Transition>
  }
}

#[component]
pub(crate) fn CachesTileDataRow(
  store_path: Signal<StorePath<String>>,
  cache: PvCache,
) -> impl IntoView {
  let download_url = format!(
    "/api/v1/c/{cache_name}/download/{store_path}",
    cache_name = cache.name,
    store_path = store_path(),
  );
  let vis_icon =
    matches!(cache.visibility, Visibility::Private).then_some(view! {
      <LockClosedHeroIcon {..} class="size-4 stroke-base-11/75 stroke-[2.0]" />
    });

  view! {
    <tr>
      <th scope="row">
        <CacheItemLink id=cache.id extra_class="text-link-primary"/>
      </th>
      <td>
        <a href={download_url} class="text-link text-link-primary">"Download"</a>
      </td>
      <td class="flex flex-row items-center gap-1">
        { cache.visibility.to_string() }
        { vis_icon }
      </td>
    </tr>
  }
}
