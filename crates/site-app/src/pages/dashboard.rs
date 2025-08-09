use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, AuthUser};

use crate::pages::UnauthorizedPage;

#[component]
pub fn DashboardPage() -> impl IntoView {
  let params = use_params_map();
  let authorized = Memo::new(move |_| {
    let allowed_orgs = use_context::<AuthUser>()
      .map(|au| au.iter_orgs().collect::<Vec<_>>())
      .unwrap_or_default();
    let Some(requested_org) = params()
      .get("org")
      .expect("missing org path param")
      .parse::<RecordId<_>>()
      .ok()
    else {
      return false;
    };
    allowed_orgs.contains(&requested_org)
  });

  move || match authorized() {
    true => view! { <DashboardInner /> }.into_any(),
    false => view! { <UnauthorizedPage /> }.into_any(),
  }
}

#[component]
fn DashboardInner() -> impl IntoView {
  view! {
    <div class="grid gap-4 h-full grid-cols-2 grid-rows-2">
      <div class="elevation-flat">
        <p class="title">"Dashboard"</p>
      </div>
    </div>
  }
}
