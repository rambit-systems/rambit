use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser};

use crate::pages::UnauthorizedPage;

#[component]
pub fn ProtectedByOrgPage(children: Children) -> impl IntoView {
  let auth_user = expect_context::<AuthUser>();
  let params = use_params_map();
  let requested_org = params()
    .get("org")
    .expect("missing org path param")
    .parse::<RecordId<_>>()
    .ok();

  match requested_org {
    Some(org) if auth_user.belongs_to_org(org) => {
      provide_context(org);
      view! { { children() } }.into_any()
    }
    Some(_) | None => view! { <UnauthorizedPage /> }.into_any(),
  }
}
