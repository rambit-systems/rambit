use leptos::{ev::keydown, prelude::*};
use leptos_use::{on_click_outside, use_event_listener, use_window};
use models::{dvf::RecordId, AuthUser, Org};

use crate::{
  components::{CheckHeroIcon, ChevronDownHeroIcon, LoadingCircle},
  hooks::OrgHook,
  navigation::navigate_to,
};

#[island]
pub(super) fn OrgSelectorPopover(user: AuthUser) -> impl IntoView {
  const CONTAINER_CLASS: &str = "hover:bg-base-3 active:bg-base-4 px-2 py-1 \
                                 rounded flex flex-col gap leading-none \
                                 items-end gap-0.5";

  let active_org = user.active_org();
  let active_org_hook = OrgHook::new(move || active_org, user.clone());
  let active_org_descriptor = active_org_hook.descriptor();

  let is_open = RwSignal::new(false);
  let popover_ref = NodeRef::<leptos::html::Div>::new();

  let toggle = move |_| {
    is_open.update(|open| {
      *open = !*open;
    })
  };

  // close on click outside
  let _ = on_click_outside(popover_ref, move |_| {
    if is_open() {
      is_open.set(false);
    }
  });

  // close on `Escape` key
  let window = use_window();
  let _ = use_event_listener(window, keydown, move |evt| {
    if evt.key() == "Escape" && is_open.get() {
      is_open.set(false);
    }
  });

  view! {
    <div class="relative">
      <div class=CONTAINER_CLASS on:click=toggle>
        <span class="text-base-12">{ user.name.to_string() }</span>
        <div class="flex flex-row items-center gap-0.5">
          <span class="text-sm">
            <Suspense fallback=|| "[loading]">
              { move || Suspend::new(active_org_descriptor) }
            </Suspense>
          </span>
          <ChevronDownHeroIcon {..} class="size-3 stroke-[3.0] stroke-base-11" />
        </div>
      </div>

      <OrgSelector
        user=user
        node_ref={popover_ref}
        {..}
        class:hidden=move || !is_open()
      />
    </div>
  }
}

#[component]
fn OrgSelector(
  user: AuthUser,
  node_ref: NodeRef<leptos::html::Div>,
) -> impl IntoView {
  const POPOVER_CLASS: &str =
    "absolute left-0 top-[calc(100%+(var(--spacing)*2))] min-w-56 \
     elevation-navbar rounded p-2 flex flex-col gap-1";

  let org_hooks = user
    .iter_orgs()
    .map(|o| (o, OrgHook::new(move || o, user.clone())))
    .collect::<Vec<_>>();
  let active_org = user.active_org();

  let action = Action::new(|o| switch_active_org(*o));
  let selected = RwSignal::new(None::<RecordId<Org>>);

  // reload on successful action
  Effect::new(move || {
    if let Some(Ok(new_org)) = action.value().get() {
      let new_dash_url = format!("/dash/{new_org}");
      navigate_to(&new_dash_url)
    }
  });

  let org_row_element = move |(id, oh): (RecordId<Org>, OrgHook)| {
    let is_active = id == active_org;
    let handler = move |_| {
      if !is_active {
        selected.set(Some(id));
        action.dispatch(id);
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
        class=("hover:bg-base-3 active:bg-base-4", id != active_org)
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
    <div class=POPOVER_CLASS node_ref=node_ref>
      { org_hooks.into_iter().map(org_row_element).collect_view() }
    </div>
  }
}

#[server(prefix = "/api/sfn")]
pub async fn switch_active_org(
  new_active_org: RecordId<Org>,
) -> Result<RecordId<Org>, ServerFnError> {
  use auth_domain::{AuthDomainService, UpdateActiveOrgError};

  let Some(auth_user) = use_context::<AuthUser>() else {
    return Err(ServerFnError::new("Unauthorized"));
  };

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
