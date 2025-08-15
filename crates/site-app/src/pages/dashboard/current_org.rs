use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, AuthUser, Org, PvOrg};

use crate::resources::org::org_query_scope;

#[component]
pub(super) fn CurrentOrgTile(org: RecordId<Org>) -> impl IntoView {
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
