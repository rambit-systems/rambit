use axum::http::{header, HeaderValue, StatusCode};
use domain::{DomainService, UpdateActiveOrgError};
use leptos::{either::Either, prelude::*};
use leptos_axum::ResponseOptions;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route,
  hooks::use_params_map, path,
};
use models::{AuthUser, Org, RecordId};

use crate::{
  components::{
    form_rejection, CheckHeroIcon, ChevronDownHeroIcon, Cog6ToothHeroIcon,
    PlusHeroIcon,
  },
  form_feedback_text::*,
  hooks::OrgHook,
};

#[component(transparent)]
pub fn OrgSelectorRoutes() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("/org_selector") view=OrgSelectorMenu />
    <Route path=path!("/org_selector/action/:id") view=OrgSelectorAction />
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
      <div class="size-5" />
    })
  };
  let action_href = format!("/org_selector/action/{}", org_hook.key()());

  view! {
    <a
      class="rounded p-2 flex flex-row gap-2 items-center transition-colors text-base-12"
      class=("font-bold", active)
      class=("cursor-pointer btn-link-secondary", !active)

      href=action_href
    >
      { icon_element }
      <span class="flex-1 text-ellipsis">
        <Suspense fallback=|| "[loading]">
          { move || Suspend::new(org_hook.descriptor())}
        </Suspense>
      </span>
    </a>
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

#[component]
fn OrgSelectorAction() -> impl IntoView {
  let params = use_params_map();
  let requested_org_param = params().get("id").expect("missing org path param");

  // fail if ID can't be parsed
  let Ok(requested_org) = requested_org_param.parse::<RecordId<_>>() else {
    return form_rejection("Could not parse requested org ID").into_any();
  };

  let future = action(requested_org);
  let future_response = move |data: &Result<RecordId<Org>, String>| match data {
    Ok(new_org) => {
      let new_org = *new_org;
      let org_hook = OrgHook::new(move || new_org);
      let target = org_hook.dashboard_url()();

      let response_options = expect_context::<ResponseOptions>();
      response_options.set_status(StatusCode::SEE_OTHER);
      response_options.insert_header(
        header::LOCATION,
        HeaderValue::from_str(&target).unwrap(),
      );

      ().into_any()
    }
    Err(t) => form_rejection(t).into_any(),
  };

  view! {
    <Await future=future blocking=true let:data>
      { future_response(data) }
    </Await>
  }
  .into_any()
}

async fn action(requested_org: RecordId<Org>) -> Result<RecordId<Org>, String> {
  let auth_user = use_context::<AuthUser>().ok_or(UNAUTHENTICATED_MESSAGE)?;
  let domain_service: DomainService = expect_context();

  Ok(
    domain_service
      .switch_active_org(auth_user.id, requested_org)
      .await
      .map_err(|e| match e {
        UpdateActiveOrgError::InvalidOrg(_) => "Could not switch to this org",
        e => {
          tracing::error!("failed to fetch org: {e}");
          INTERNAL_ERROR_MESSAGE
        }
      })?,
  )
}
