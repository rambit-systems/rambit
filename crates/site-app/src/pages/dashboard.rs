mod cache;
mod entry;
mod store;

use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser, Org, PvOrg};

use self::{cache::CacheTable, entry::EntryTable, store::StoreTable};
use crate::{pages::UnauthorizedPage, resources::org::org_query_scope};

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
fn CurrentOrgTile(org: RecordId<Org>) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();
  let auth_user = Signal::stored(expect_context::<AuthUser>());

  let resource = query_client.resource(org_query_scope(), move || org);
  let org_title = move |r: Result<Option<PvOrg>, ServerFnError>| {
    r.map(|o| {
      o.and_then(|o| o.user_facing_title(&auth_user()))
        .unwrap_or("[unknown-org]".to_owned())
    })
    .unwrap_or("[error]".to_owned())
  };

  view! {
    <div class="w-80 p-4 elevation-flat flex flex-col gap-4">
      <div class="flex flex-col leading-none">
        <p class="text-xl">"org"</p>
        <p class="text-3xl text-base-12">
          <Suspense fallback=|| "[loading]">
            { move || Suspend::new(async move { org_title(resource.await) }) }
          </Suspense>
        </p>
      </div>
    </div>
  }
}

#[component]
fn DashboardInner(org: RecordId<Org>) -> impl IntoView {
  view! {
    <div class="flex flex-row items-start gap-4">
      <CurrentOrgTile org=org />
      <div class="flex-1 grid gap-4 grid-cols-2">
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
    </div>
  }
}
