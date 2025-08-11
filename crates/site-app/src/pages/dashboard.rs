use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser, Cache, Org};

use crate::pages::UnauthorizedPage;

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
  let caches_in_org = crate::resources::caches_in_org(org);
  let suspend = move || {
    Suspend::new(async move {
      match caches_in_org.await {
        Ok(caches) => view! { <CacheTable caches=caches /> }.into_any(),
        Err(e) => view! { { format!("Error: {e}") } }.into_any(),
      }
    })
  };

  view! {
    <div class="grid gap-4 h-full grid-cols-2 grid-rows-2">
      <div class="p-6 elevation-flat flex flex-col gap-4">
        <p class="title">"Caches"</p>

        <Suspense fallback=move || view! { "Loading..." }>
          { suspend }
        </Suspense>
      </div>
    </div>
  }
}

#[component]
fn CacheTable(caches: Vec<Cache>) -> impl IntoView {
  view! {
    <table class="table">
      <thead>
        <th>"Name"</th>
        <th>"Visibility"</th>
        <th>"Default Cache"</th>
      </thead>
      <tbody>
        { move || caches.iter().map(|c| view! {
          <tr>
            <th scope="row">{ c.name.to_string() }</th>
            <td>{ c.visibility.to_string() }</td>
            <td>{ c.default_store.to_string() }</td>
          </tr>
        }).collect::<Vec<_>>() }
      </tbody>
    </table>
  }
}
