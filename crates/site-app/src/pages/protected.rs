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
  // fail if use is not logged in
  let Some(auth_user) = use_context::<AuthUser>() else {
    return view! { <UnauthorizedPage /> }.into_any();
  };

  // panic if org param is not found for route
  let params = use_params_map();
  let requested_org_param =
    params().get("org").expect("missing org path param");

  // fail if ID can't be parsed
  let Ok(requested_org) = requested_org_param.parse::<RecordId<_>>() else {
    return view! { <UnauthorizedPage /> }.into_any();
  };

  // fail if user doesn't belong to org
  if !auth_user.belongs_to_org(requested_org) {
    return view! { <UnauthorizedPage /> }.into_any();
  }

  provide_context(RequestedOrg(requested_org));
  view! {
    <RequestedOrgIslandContextProvider org=requested_org children=children />
  }
  .into_any()
}

#[island]
fn RequestedOrgIslandContextProvider(
  org: RecordId<Org>,
  children: Children,
) -> impl IntoView {
  provide_context(RequestedOrg(org));
  children()
}
