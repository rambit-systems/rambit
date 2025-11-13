use std::future::IntoFuture;

use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_router::hooks::use_params_map;
use models::{AuthUser, Org, RecordId};

use crate::{
  pages::UnauthorizedPage, resources::org::self_is_org_owner_query_scope,
};

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

pub fn protect_by_org_owner<
  F: Fn() -> O + Send + Sync + Copy + 'static,
  O: IntoView + 'static,
>(
  func: F,
) -> impl Send + Clone + 'static + Fn() -> impl IntoAny {
  move || view! { <ProtectedByOrgOwnerPage> { func() } </ProtectedByOrgOwnerPage> }
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

#[component]
pub fn ProtectedByOrgOwnerPage(children: Children) -> impl IntoView {
  // fail if use is not logged in
  let Some(_auth_user) = use_context::<AuthUser>() else {
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

  // query for the org
  let key_fn = move || requested_org;
  let query_scope = self_is_org_owner_query_scope();
  let resource =
    expect_context::<QueryClient>().resource_blocking(query_scope, key_fn);

  let children = move |data: &Result<bool, ServerFnError>| match data {
    Ok(true) => {
      provide_context(RequestedOrg(requested_org));
      view! {
        <RequestedOrgIslandContextProvider org=requested_org children=children />
      }.into_any()
    }
    Ok(false) => view! { <UnauthorizedPage /> }.into_any(),
    Err(e) => e.to_string().into_any(),
  };

  view! {
    <Await blocking=true future={resource.into_future()} children=children />
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
