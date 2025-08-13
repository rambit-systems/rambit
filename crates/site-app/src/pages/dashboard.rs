mod cache;
mod entry;
mod store;

use std::{fmt::Debug, hash::Hash};

use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser, Org};
use serde::{de::DeserializeOwned, Serialize};

use self::{cache::CacheTable, entry::EntryTable, store::StoreTable};
use crate::{components::LoadingCircle, pages::UnauthorizedPage};

#[component]
pub fn DashboardPage() -> impl IntoView {
  let params = use_params_map();
  let authorized_org = Memo::new(move |_| {
    let allowed_orgs = use_context::<AuthUser>()
      .map(|au| au.iter_orgs().collect::<Vec<_>>())
      .unwrap_or_default();
    let requested_org = params()
      .get("org")
      .expect("missing org path param")
      .parse::<RecordId<_>>()
      .ok()?;
    allowed_orgs
      .contains(&requested_org)
      .then_some(requested_org)
  });

  move || match authorized_org() {
    Some(org) => view! { <DashboardInner org=org /> }.into_any(),
    None => view! { <UnauthorizedPage /> }.into_any(),
  }
}

#[component]
fn DashboardInner(org: RecordId<Org>) -> impl IntoView {
  view! {
    <div class="grid gap-4 h-full grid-cols-2 grid-rows-2">
      <div class="col-span-2 p-6 elevation-flat flex flex-col gap-4">
        <EntryTable org=org />
      </div>
      <div class="p-6 elevation-flat flex flex-col gap-4">
        <CacheTable org=org />
      </div>
      <div class="p-6 elevation-flat flex flex-col gap-4">
        <StoreTable org=org />
      </div>
    </div>
  }
}

#[component]
fn DataTable<
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
fn DataTableRefreshButton<
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
    <button class="btn-link btn-link-secondary relative duration-300" on:click=invalidate>
      <span class="transition-opacity" class=("opacity-0", fetching)>"Refresh"</span>
      <div class="absolute inset-0 flex flex-row justify-center items-center">
        <LoadingCircle {..} class="size-5 transition-opacity" class=("opacity-0", move || { !fetching() }) />
      </div>
    </button>
  }
}
