use leptos::{either::Either, prelude::*};
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::AuthUser;

use crate::{
  components::{
    CheckHeroIcon, ChevronDownHeroIcon, Cog6ToothHeroIcon, LoadingCircle,
    PlusHeroIcon,
  },
  hooks::OrgHook,
};

#[component(transparent)]
pub fn OrgSelectorRoutes() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("/org_selector") view=OrgSelectorMenu />
  }
  .into_inner()
  .into_any_nested_route()
}

#[component]
pub fn OrgSelector() -> impl IntoView {
  view! {
    <RemovePopoverOnClickOutside />
    <OrgSelectorTrigger />
  }
}

#[component]
fn RemovePopoverOnClickOutside() -> impl IntoView {
  const SCRIPT: &str = r##"
    document.addEventListener('click', function(e) {
        const popover = document.querySelector('[data-popover]');
        const trigger = document.querySelector('[hx-target="#org-selector-popover-contents"]');
    
        if (popover && !popover.contains(e.target) && !trigger.contains(e.target)) {
            popover.remove();
        }
    });
  "##;

  view! {
    <script>{ SCRIPT }</script>
  }
}

#[component]
fn OrgSelectorTrigger() -> impl IntoView {
  let active_org_hook = OrgHook::new_active();
  let active_org_descriptor = active_org_hook.descriptor();

  const CLASS: &str = "transition-colors hover:bg-base-3 active:bg-base-4 \
                       cursor-pointer px-2 py-1 rounded flex flex-col gap-0.5 \
                       text-sm leading-none items-end gap-0 relative";

  view! {
    <div
      class=CLASS
      hx-get="/org_selector"
      hx-target="#org-selector-popover-contents"
      hx-swap="innerHTML transition:true"
    >
      <p class="text-base/[1] text-base-12">
        <Suspense fallback=|| "[loading]">
          { move || Suspend::new(active_org_descriptor) }
        </Suspense>
      </p>
      <div class="flex flex-row items-end gap-0.5">
        <p>"Switch Orgs"</p>
        <ChevronDownHeroIcon {..} class="size-3 stroke-[3.0] stroke-base-11" />
      </div>

      <div id="org-selector-popover-contents" class="contents" />
    </div>
  }
}

#[component]
fn OrgSelectorMenu() -> impl IntoView {
  let auth_user = expect_context::<AuthUser>();

  const POPOVER_CLASS: &str =
    "absolute right-0 top-[calc(100%+(var(--spacing)*4))] min-w-56 \
     elevation-lv1 z-50 p-2 flex flex-col gap-1 leading-none";

  let active_org = auth_user.active_org();
  let org_rows = auth_user.iter_orgs().map(|o| {
    view! {
      <OrgRow org_hook={OrgHook::new(move || o)} active={o == active_org} />
    }
  });

  view! {
    <div
      data-popover
      class=POPOVER_CLASS
    >
      { org_rows.collect_view() }
      <div class="p-1">
        <div class="h-0 border-t-2 border-base-6/75" />
      </div>
      <ExtraRows />
    </div>
  }
}

#[component]
fn OrgRow(org_hook: OrgHook, active: bool) -> impl IntoView {
  let icon_element = if active {
    Either::Left(view! {
      <CheckHeroIcon {..} class="size-5 stroke-product-11 stroke-[2.0]" />
    })
  } else {
    Either::Right(view! {
      <LoadingCircle {..} class="size-5 invisible" />
    })
  };

  view! {
    <div
      class="rounded p-2 flex flex-row gap-2 items-center transition-colors text-base-12"
      class=("font-bold", active)
      class=("cursor-pointer btn-link-secondary", !active)
    >
      { icon_element }
      <span class="flex-1 text-ellipsis">
        <Suspense fallback=|| "[loading]">
          { move || Suspend::new(org_hook.descriptor())}
        </Suspense>
      </span>
    </div>
  }
}

#[component]
fn ExtraRows() -> impl IntoView {
  let active_org_hook = OrgHook::new_active();
  let active_org_settings_url = active_org_hook.settings_url();

  view! {
    <a href=active_org_settings_url class="btn-link btn-link-secondary btn-link-tight">
      <Cog6ToothHeroIcon {..} class="size-5 stroke-base-11 stroke-[2.0]" />
      <span class="flex-1 text-ellipsis">
        "Org Settings"
      </span>
    </a>
    <a href="/org/create_org" class="btn-link btn-link-secondary btn-link-tight">
      <PlusHeroIcon {..} class="size-5 stroke-base-11 stroke-[2.0]" />
      <span class="flex-1 text-ellipsis">
        "Create Org"
      </span>
    </a>
  }
}
