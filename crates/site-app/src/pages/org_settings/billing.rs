use leptos::prelude::*;
use leptos_fetch::QueryClient;

use crate::hooks::OrgHook;

#[component]
pub fn OrgSettingsSubPageBilling() -> impl IntoView {
  let org_hook = OrgHook::new_requested();

  let query_client = expect_context::<QueryClient>();
  let org_sub_report_key_fn = org_hook.key();
  let org_sub_report_query_scope =
    crate::resources::subscription::org_subscription_report_query_scope();
  let org_sub_report_resource =
    query_client.resource(org_sub_report_query_scope, org_sub_report_key_fn);

  let suspend_fn = move || {
    Suspend::new(async move { format!("{:#?}", org_sub_report_resource.await) })
  };

  view! {
    <p class="subtitle">"Billing"</p>

    <code class="overflow-auto whitespace-pre">
      <Transition fallback=|| "loading">
        { suspend_fn }
      </Transition>
    </code>
  }
}
