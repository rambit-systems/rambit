use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use serde::{de::DeserializeOwned, Serialize};

use crate::components::LoadingCircle;

#[component]
pub fn DataTable<
  K: Clone + Hash + PartialEq + Debug + Send + Sync + 'static,
  KF: Fn() -> K + Copy + Send + Sync + 'static,
  O: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
  VF: Fn(Signal<Vec<O>>) -> V + Copy + Send + Sync + 'static,
  V: IntoView,
>(
  key_fn: KF,
  query_scope: QueryScope<K, Result<Vec<O>, ServerFnError>>,
  view_fn: VF,
) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let resource = query_client.local_resource(query_scope, key_fn);

  view! {
    <Transition fallback=|| ()>
      { move || Suspend::new(async move { match resource.await {
        Ok(output) => view_fn(Signal::stored(output)).into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }})}
    </Transition>
  }
}

#[component]
pub fn DataTableRefreshButton<
  K: Clone + Hash + PartialEq + Debug + Send + Sync + 'static,
  KF: Fn() -> K + Copy + Send + Sync + 'static,
  O: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
>(
  key_fn: KF,
  query_scope: QueryScope<K, Result<Vec<O>, ServerFnError>>,
) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let fetching =
    query_client.subscribe_is_fetching(query_scope.clone(), key_fn);
  let invalidate = {
    let query_scope = query_scope.clone();
    move |_| {
      query_client.invalidate_query(query_scope.clone(), key_fn());
    }
  };

  view! {
    <button class="btn btn-secondary relative duration-300" on:click=invalidate>
      <span class="transition-opacity" class=("opacity-0", fetching)>"Refresh"</span>
      <div class="absolute inset-0 flex flex-row justify-center items-center">
        <LoadingCircle {..} class="size-5 transition-opacity" class=("opacity-0", move || { !fetching() }) />
      </div>
    </button>
  }
}
