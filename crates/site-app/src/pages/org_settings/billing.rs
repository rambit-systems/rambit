use leptos::prelude::*;
use site_paddle::PaddleProvider;

#[component]
pub fn OrgSettingsSubPageBilling() -> impl IntoView {
  view! {
    <PaddleProvider>
      <p class="subtitle">"Billing"</p>
    </PaddleProvider>
  }
}
