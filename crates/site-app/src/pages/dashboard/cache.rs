use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Cache, Org};

use crate::{
  components::{CacheItemLink, LoadingCircle, StoreItemLink},
  resources::cache::caches_in_org_query_scope,
};

#[island]
pub(super) fn CacheTable(org: RecordId<Org>) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let key = move || org;
  let caches_in_org = crate::resources::cache::caches_in_org(key);
  let fetching =
    query_client.subscribe_is_fetching(caches_in_org_query_scope(), key);
  let invalidate = move |_| {
    query_client.invalidate_query(caches_in_org_query_scope(), key());
  };

  let suspend = move || {
    Suspend::new(async move {
      match caches_in_org.await {
        Ok(caches) => view! { <CacheDataTable caches=caches /> }.into_any(),
        Err(e) => view! { { format!("Error: {e}") } }.into_any(),
      }
    })
  };

  view! {
    <div class="flex flex-row items-start gap-2">
      <p class="title">"Caches"</p>
      <div class="flex-1" />
      <button class="btn btn-secondary relative" on:click=invalidate>
        <span class:invisible=fetching>"Reload"</span>
        <div class="absolute inset-0 flex flex-row justify-center items-center">
          <LoadingCircle {..} class="size-6" class:invisible=move || { !fetching() }/>
        </div>
      </button>
    </div>

    <Suspense fallback=move || view! { "Loading..." }>
      { suspend }
    </Suspense>
  }
}

#[component]
fn CacheDataTable(caches: Vec<Cache>) -> impl IntoView {
  view! {
    <table class="table">
      <thead>
        <th>"Name"</th>
        <th>"Visibility"</th>
        <th>"Default Store"</th>
      </thead>
      <tbody>
        { move || caches.iter().map(|c| view! {
          <tr>
            <th scope="row"><CacheItemLink id=c.id {..} class="btn-link-primary"/></th>
            <td>{ c.visibility.to_string() }</td>
            <td><StoreItemLink id=c.default_store /></td>
          </tr>
        }).collect::<Vec<_>>() }
      </tbody>
    </table>
  }
}
