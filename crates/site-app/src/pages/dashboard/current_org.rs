use leptos::prelude::*;
use models::{dvf::RecordId, Org};

use crate::hooks::OrgHook;

#[component]
pub(super) fn CurrentOrgTile(org: RecordId<Org>) -> impl IntoView {
  let org_hook = OrgHook::new(move || org);
  let descriptor = org_hook.descriptor();

  view! {
    <div class="p-4 elevation-flat flex flex-col gap-4">
      <div class="flex flex-col leading-none">
        <p class="text-xl">"org"</p>
        <p class="text-3xl text-base-12">
          <Suspense fallback=|| "[loading]">
            { move || Suspend::new(descriptor) }
          </Suspense>
        </p>
      </div>
    </div>
  }
}
