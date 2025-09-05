use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use serde::{de::DeserializeOwned, Serialize};

use crate::components::LoadingCircle;

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

#[component]
pub fn TableEmptyBody(children: Children) -> impl IntoView {
  view! {
    <tr class="animate-fade-in h-20 relative border-[2px] border-t-0 border-base-6 border-dashed rounded-b">
      <td></td><td></td><td></td><td></td>
      <div class="absolute inset-0 flex flex-col items-center justify-center">
        { children() }
      </div>
    </tr>
  }
}
