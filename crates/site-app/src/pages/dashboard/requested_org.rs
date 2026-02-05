use leptos::prelude::*;

use crate::{hooks::OrgHook, join_classes::JoinClasses};

#[component]
pub(super) fn RequestedOrgTile(
  #[prop(optional)] class: Option<impl AsRef<str>>,
) -> impl IntoView {
  const CLASS: &str = "p-4 elevation-flat flex flex-col gap-4";

  let class = class.as_ref().map(|a| a.as_ref());
  let class = [CLASS, class.unwrap_or_default()].join_classes();

  let org_hook = OrgHook::new_requested();
  let descriptor = org_hook.descriptor();

  view! {
    <div class=class>
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
