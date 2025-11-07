use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{AuthUser, Org, RecordId};

use crate::pages::UnauthorizedPage;

pub fn protect<
  F: Fn() -> O + Send + Sync + Copy + 'static,
  O: IntoView + 'static,
>(
  func: F,
) -> impl Send + Clone + 'static + Fn() -> impl IntoAny {
  move || view! { <ProtectedPage> { func() } </ProtectedPage> }
}

pub fn protect_by_org<
  F: Fn() -> O + Send + Sync + Copy + 'static,
  O: IntoView + 'static,
>(
  func: F,
) -> impl Send + Clone + 'static + Fn() -> impl IntoAny {
  move || view! { <ProtectedByOrgPage> { func() } </ProtectedByOrgPage> }
}

#[component]
pub fn ProtectedPage(children: Children) -> impl IntoView {
  match use_context::<AuthUser>() {
    Some(_) => children(),
    None => view! { <UnauthorizedPage /> }.into_any(),
  }
}

#[derive(Copy, Clone, Debug)]
pub struct RequestedOrg(pub RecordId<Org>);

#[component]
pub fn ProtectedByOrgPage(children: Children) -> impl IntoView {
  let auth_user = use_context::<AuthUser>();
  let params = use_params_map();
  let requested_org = params()
    .get("org")
    .expect("missing org path param")
    .parse::<RecordId<_>>()
    .ok();

  match requested_org {
    Some(org) if auth_user.is_some_and(|u| u.belongs_to_org(org)) => {
      provide_context(RequestedOrg(org));
      view! {
        <RequestedOrgIslandContextProvider org=org children=children />
      }
      .into_any()
    }
    Some(_) | None => view! { <UnauthorizedPage /> }.into_any(),
  }
}

#[island]
fn RequestedOrgIslandContextProvider(
  org: RecordId<Org>,
  children: Children,
) -> impl IntoView {
  provide_context(RequestedOrg(org));
  children()
}
