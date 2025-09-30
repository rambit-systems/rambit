use leptos::prelude::*;
use models::{dvf::RecordId, AuthUser, Org};

use crate::{
  components::{
    CheckHeroIcon, ChevronDownHeroIcon, LoadingCircle, PlusHeroIcon, Popover,
    PopoverContents, PopoverTrigger,
  },
  hooks::OrgHook,
  navigation::navigate_to,
};

#[component]
fn OrgSelectorTrigger() -> impl IntoView {
  let active_org_hook = OrgHook::new_active();
  let active_org_descriptor = active_org_hook.descriptor();

  const CLASS: &str = "transition hover:bg-base-3 active:bg-base-4 \
                       cursor-pointer px-2 py-1 rounded flex flex-col gap \
                       leading-none items-end gap-0";

  view! {
    <div class=CLASS>
      <span class="text-base-12 text-sm">
        <Suspense fallback=|| "[loading]">
          { move || Suspend::new(active_org_descriptor) }
        </Suspense>
      </span>
      <div class="flex flex-row items-center gap-0">
        <span class="text-sm">"Switch Orgs"</span>
        <ChevronDownHeroIcon {..} class="size-3 stroke-[3.0] stroke-base-11" />
      </div>
    </div>
  }
}

#[island]
pub(super) fn OrgSelector() -> impl IntoView {
  view! {
    <Popover>
      <PopoverTrigger slot>
        <OrgSelectorTrigger />
      </PopoverTrigger>

      <PopoverContents slot>
        <OrgSelectorMenu />
      </PopoverContents>
    </Popover>
  }
}

#[component]
fn OrgSelectorMenu() -> impl IntoView {
  let auth_user = expect_context::<AuthUser>();

  const POPOVER_CLASS: &str =
    "absolute left-0 top-[calc(100%+(var(--spacing)*2))] min-w-56 \
     elevation-lv1 p-2 flex flex-col gap-1 leading-none";

  let org_hooks = Signal::stored(
    auth_user
      .iter_orgs()
      .map(|o| (o, OrgHook::new(move || o)))
      .collect::<Vec<_>>(),
  );
  let active_org = auth_user.active_org();

  let action = ServerAction::<SwitchActiveOrg>::new();
  let selected = RwSignal::new(None::<RecordId<Org>>);

  // reload on successful action
  Effect::new(move || {
    if let Some(Ok(new_org)) = action.value().get() {
      let org_hook = org_hooks
        .get()
        .into_iter()
        .find(|(o, _)| new_org == *o)
        .expect("failed to find new org's hook")
        .1;
      let new_dash_url = org_hook.dashboard_url()();
      navigate_to(&new_dash_url)
    }
  });

  let org_row_element = move |(id, oh): (RecordId<Org>, OrgHook)| {
    let is_active = id == active_org;
    let handler = move |_| {
      if !is_active {
        selected.set(Some(id));
        action.dispatch(SwitchActiveOrg { new_active_org: id });
      }
    };

    let icon_element = if is_active {
      view! {
        <CheckHeroIcon {..} class="size-5 stroke-product-11 stroke-[2.0]" />
      }
      .into_any()
    } else {
      view! {
        <LoadingCircle {..} class="size-5" class:invisible=move || selected.get() != Some(id) />
      }
      .into_any()
    };

    view! {
      <div
        class="rounded p-2 flex flex-row gap-2 items-center"
        class=("text-base-12 font-semibold", id == active_org)
        class=("cursor-pointer hover:bg-base-3 active:bg-base-4", id != active_org)
        on:click=handler
      >
        { icon_element }
        <span class="flex-1 text-ellipsis">
          <Suspense fallback=|| "[loading]">
            { move || Suspend::new(oh.descriptor())}
          </Suspense>
        </span>
      </div>
    }
  };

  view! {
    <div
      class=POPOVER_CLASS
    >
      { org_hooks().into_iter().map(org_row_element).collect_view() }
      <div class="p-1">
        <div class="h-0 border-t-2 border-base-6/75" />
      </div>
      <CreateOrgRow />
    </div>
  }
}

#[component]
fn CreateOrgRow() -> impl IntoView {
  const CLASS: &str = "rounded p-2 flex flex-row gap-2 items-center \
                       cursor-pointer hover:bg-base-3 active-bg-base-4";

  view! {
    <a href="/org/create_org" class=CLASS>
      <PlusHeroIcon {..} class="size-5 stroke-product-11 stroke-[2.0]" />
      <span class="flex-1 text-ellipsis">
        "Create Organization..."
      </span>
    </a>
  }
}

#[server(prefix = "/api/sfn")]
pub async fn switch_active_org(
  new_active_org: RecordId<Org>,
) -> Result<RecordId<Org>, ServerFnError> {
  use auth_domain::{AuthDomainService, UpdateActiveOrgError};

  let auth_user = crate::resources::authenticate()?;

  let auth_domain_service: AuthDomainService = expect_context();

  auth_domain_service
    .switch_active_org(auth_user.id, new_active_org)
    .await
    .map_err(|e| match e {
      UpdateActiveOrgError::InvalidOrg(record_id) => {
        ServerFnError::new(format!("invalid org: {record_id}"))
      }
      e => {
        tracing::error!("failed to fetch org: {e}");
        ServerFnError::new("internal error")
      }
    })
}
