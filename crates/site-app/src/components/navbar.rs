use leptos::{either::Either, ev::keydown, prelude::*};
use leptos_use::{on_click_outside, use_event_listener, use_window};
use models::AuthUser;

use crate::{
  components::{CheckHeroIcon, ChevronDownHeroIcon},
  hooks::OrgHook,
  navigation::next_url_hook,
};

#[component]
pub fn Navbar() -> impl IntoView {
  let class = "elevation-navbar flex flex-row px-4 gap-2 items-center h-14 \
               rounded-bl rounded-br mb-8";

  view! {
    <div class=class >
      <a href="/" class="cursor-pointer font-display font-bold text-xl text-product-11">
        "Rambit"
      </a>
      <div class="flex-1" />
      <NavbarUserArea />
    </div>
  }
}
#[component]
fn NavbarUserArea() -> impl IntoView {
  let auth_user = use_context::<AuthUser>();

  match auth_user {
    Some(user) => Either::Left(view! { <LoggedInUserAuthActions user=user /> }),
    None => Either::Right(view! { <LoggedOutUserAuthActions /> }),
  }
}

#[component]
fn LoggedOutUserAuthActions() -> impl IntoView {
  let next_url = next_url_hook();
  let signup_url =
    Signal::derive(move || format!("/auth/signup?next={}", next_url()));
  let login_url =
    Signal::derive(move || format!("/auth/login?next={}", next_url()));

  view! {
    <div class="flex flex-row gap-1 items-center">
      <a href=login_url class="btn-link btn-link-secondary">"Log In"</a>
      <a href=signup_url class="btn-link btn-link-primary">"Sign Up"</a>
    </div>
  }
}

#[component]
fn LoggedInUserAuthActions(user: AuthUser) -> impl IntoView {
  let active_org = user.active_org();
  let active_org_hook = OrgHook::new(move || active_org, user.clone());
  let active_org_dashboard_url = active_org_hook.dashboard_url();

  view! {
    <OrgSelectorPopover user=user />
    <div class="flex flex-row gap-1 items-center">
      <a href=active_org_dashboard_url class="btn-link btn-link-primary">"Dashboard"</a>
      <a href="/auth/logout" class="btn-link btn-link-secondary">"Log Out"</a>
    </div>
  }
}

#[island]
fn OrgSelectorPopover(user: AuthUser) -> impl IntoView {
  const CONTAINER_CLASS: &str = "relative hover:bg-base-3 active:bg-base-4 \
                                 px-2 py-1 rounded flex flex-col gap \
                                 leading-none items-end gap-0.5";

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

      <OrgSelector
        user={user}
        {..}
        class:hidden=move || !is_open()
        node_ref=popover_ref
      />
    </div>
  }
}

#[island]
fn OrgSelector(user: AuthUser) -> impl IntoView {
  const POPOVER_CLASS: &str =
    "absolute left-0 top-[calc(100%+(var(--spacing)*2))] min-w-56 \
     elevation-navbar rounded p-2 flex flex-col gap-1";

  let org_hooks = user
    .iter_orgs()
    .map(|o| (o, OrgHook::new(move || o, user.clone())))
    .collect::<Vec<_>>();
  let active_org = user.active_org();

  view! {
    <div class=POPOVER_CLASS>
      { org_hooks.into_iter().map(move |(id, oh)| view! {
        <div class="hover:bg-base-3 active:bg-base-4 rounded p-2 flex flex-row gap-2 items-center">
          <CheckHeroIcon {..}
            class="size-5 stroke-product-11 stroke-[2.0]"
            class:invisible={id != active_org}
          />
          <span class="flex-1 text-ellipsis">
            <Suspense fallback=|| "[loading]">
              { move || Suspend::new(oh.descriptor())}
            </Suspense>
          </span>
        </div>
      }).collect_view()}
    </div>
  }
}
