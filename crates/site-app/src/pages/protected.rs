use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser, Org};

use crate::pages::UnauthorizedPage;

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
