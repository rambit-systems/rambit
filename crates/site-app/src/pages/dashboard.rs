mod cache;
mod entry;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser, Org};

use self::{cache::CacheTable, entry::EntryTable};
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
