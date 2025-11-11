use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn OrgSettingsPage() -> impl IntoView {
  const CONTAINER_CLASS: &str =
    "grid grid-cols-[320px_minmax(0,_1fr)] gap-4 place-items-start";

  view! {
    <div class=CONTAINER_CLASS>
      <p class="title xl:col-start-2">"Org Settings"</p>
      <Outlet />
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
