use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::{
  components::{Cog6ToothHeroIcon, CreditCardHeroIcon},
  hooks::OrgHook,
};

#[component]
pub fn OrgSettingsPage() -> impl IntoView {
  const CONTAINER_CLASS: &str = r#"grid grid-cols-[calc(64*var(--spacing))_minmax(0,_1fr)] gap-4 place-items-start"#;

  view! {
    <div class=CONTAINER_CLASS>
      <div class="col-start-2">
        <p class="title">"Org Settings"</p>
      </div>
      <NavButtons />
      <div class="elevation-flat w-full p-8 gap-8">
        <Outlet />
      </div>
    </div>
  }
}

#[component]
pub fn NavButtons() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let settings_url = org_hook.settings_url();
  let settings_billing_url =
    Signal::derive(move || format!("{}/billing", settings_url()));

  view! {
    <div class="elevation-flat p-4 flex flex-col gap-2 w-64">
      <a href=settings_url class="btn btn-secondary btn-tight">
        <Cog6ToothHeroIcon {..} class="size-5 stroke-base-11 stroke-[2.0]" />
        "Overview"
      </a>
      <a href=settings_billing_url class="btn btn-secondary btn-tight">
        <CreditCardHeroIcon {..} class="size-5 stroke-base-11 stroke-[2.0]" />
        "Billing"
      </a>
    </div>
  }
}

#[component]
pub fn OrgSettingsSubPageOverview() -> impl IntoView {
  view! {
    <p class="subtitle">"Overview"</p>
  }
}

#[component]
pub fn OrgSettingsSubPageBilling() -> impl IntoView {
  view! {
    <p class="subtitle">"Billing"</p>
  }
}
