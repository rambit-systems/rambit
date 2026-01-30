use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::EntityName;

use super::NAME_FIELD_NAME;
use crate::resources::org::org_name_is_available_query_scope;

#[component]
pub(super) fn OrgNameValidation() -> impl IntoView {
  let form_data = leptos_router::hooks::use_query_map().get();
  let Some(name) = form_data.get(NAME_FIELD_NAME) else {
    return const_format::formatcp!(
      "The validation request did not contain the \"{NAME_FIELD_NAME}\" field \
       :/"
    )
    .into_any();
  };

  if name.is_empty() {
    return ().into_any();
  }

  let sanitized_name = EntityName::new(name.clone());

  let sanitization_hint = (sanitized_name.as_ref() != name).then_some(view! {
    <p class="text-warn-11 text-sm">
      "This name will be converted to \""{ sanitized_name.as_ref() }"\"".
    </p>
  });

  let is_available_resource = expect_context::<QueryClient>().resource(
    org_name_is_available_query_scope(),
    Signal::stored(sanitized_name.to_string()),
  );
  let availability_hint_suspend = move || {
    Suspend::new(async move {
      match is_available_resource.await {
        Ok(true) => view! {
          <p class="text-sm">
            "This name is available."
          </p>
        }
        .into_any(),
        Ok(false) => view! {
          <p class="text-critical-11 text-sm">
            "This name is not available."
          </p>
        }
        .into_any(),
        Err(_) => view! {
          <p class="text-critical-11 text-sm">
            "Unable to check org name availability"
          </p>
        }
        .into_any(),
      }
    })
  };

  view! {
    <div class="flex flex-col">
      { sanitization_hint }
      { availability_hint_suspend }
    </div>
  }
  .into_any()
}
