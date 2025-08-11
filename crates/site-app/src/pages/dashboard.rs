mod cache;
mod entry;

use std::{fmt::Debug, future::Future, hash::Hash};

use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser, Org};
use serde::{de::DeserializeOwned, Serialize};

use self::{cache::CacheTable, entry::EntryTable};
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
    </div>
  }
}

#[component]
fn DataTable<
  K: Clone + Hash + PartialEq + Debug + Send + Sync + 'static,
  KF: Fn() -> K + Copy + Send + Sync + 'static,
  OF: Fn(K) -> Fut + Copy + Send + Sync + 'static,
  Fut: Future<Output = Result<Vec<O>, ServerFnError>> + Send + 'static,
  O: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
  VF: Fn(Vec<O>) -> V + Copy + Send + Sync + 'static,
  V: IntoView,
>(
  key_fn: KF,
  fetcher: OF,
  view_fn: VF,
) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let query_scope = QueryScope::new(fetcher);
  let resource = query_client.resource(query_scope, key_fn);

  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(output) => view_fn(output).into_any(),
        Err(e) => view! { { format!("Error: {e}") } }.into_any(),
      }
    })
  };

  view! {
    <Suspense fallback=move || view! { "Loading..." }>
      { suspend }
    </Suspense>
  }
}

#[component]
fn DataTableReloadButton<
  K: Clone + Hash + PartialEq + Debug + Send + Sync + 'static,
  KF: Fn() -> K + Copy + Send + Sync + 'static,
  OF: Fn(K) -> Fut + Send + Sync + 'static,
  Fut: Future<Output = Result<Vec<O>, ServerFnError>> + Send + 'static,
  O: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
>(
  key_fn: KF,
  fetcher: OF,
) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let query_scope = QueryScope::new(fetcher);
  let fetching =
    query_client.subscribe_is_fetching(query_scope.clone(), key_fn);
  let invalidate = {
    let query_scope = query_scope.clone();
    move |_| {
      query_client.invalidate_query(query_scope.clone(), key_fn());
    }
  };

  view! {
    <button class="btn btn-secondary relative" on:click=invalidate>
      <span class:invisible=fetching>"Reload"</span>
      <div class="absolute inset-0 flex flex-row justify-center items-center">
        <LoadingCircle {..} class="size-6" class:invisible=move || { !fetching() }/>
      </div>
    </button>
  }
}
