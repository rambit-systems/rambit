use leptos::prelude::*;

use crate::hooks::OrgHook;

#[component]
pub(super) fn RequestedOrgTile() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let descriptor = org_hook.descriptor();

  view! {
    <div class="p-4 elevation-flat flex flex-col gap-4">
      <div class="flex flex-col leading-none">
        <div class="flex flex-row gap-2 items-center justify-between">
          <p class="text-xl">"org"</p>
          <a
            href="/org/create_org"
            class="text-link text-link-primary"
          >"Create Org..."</a>
        </div>
        <p class="text-3xl text-base-12">
          <Suspense fallback=|| "[loading]">
            { move || Suspend::new(descriptor) }
          </Suspense>
        </p>
      </div>
    </div>
  }
}
