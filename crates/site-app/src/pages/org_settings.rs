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
  let current_url = leptos_router::hooks::use_url();
  let current_path = Signal::derive(move || current_url().path().to_owned());

  let settings_url = org_hook.settings_url();
  let settings_billing_url =
    Signal::derive(move || format!("{}/billing", settings_url()));

  let is_generator =
    move |s: Signal<String>| Signal::derive(move || current_path() == s());
  let is_overview = is_generator(settings_url.into());
  let is_billing = is_generator(settings_billing_url);

  let class_generator = move |is: Signal<bool>| {
    Signal::derive(move || {
      if is() {
        "btn btn-secondary btn-tight"
      } else {
        "btn-link btn-link-secondary btn-link-tight"
      }
    })
  };
  let overview_class = class_generator(is_overview);
  let billing_class = class_generator(is_billing);

  view! {
    <div class="elevation-flat p-4 flex flex-col gap-2 w-64">
      <a href=settings_url class=overview_class>
        <Cog6ToothHeroIcon {..} class="size-5 stroke-base-11 stroke-[2.0]" />
        "Overview"
      </a>
      <a href=settings_billing_url class=billing_class>
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
