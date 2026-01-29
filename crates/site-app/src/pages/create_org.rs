use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::EntityName;

use crate::{
  components::{form_layout::*, BuildingOffice2HeroIcon, LoadingCircle},
  pages::protect,
  resources::org::org_name_is_available_query_scope,
};

const ORG_DESCRIPTION: &str =
  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
   tempor incididunt ut labore et dolore magna aliqua.

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
   aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
   voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

#[component(transparent)]
pub fn CreateOrgPageRoutes() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("/org/create_org") view=protect(CreateOrgPage) />
    <Route path=path!("/org/create_org/validate") view=protect(OrgNameValidation) />
  }
  .into_inner()
  .into_any_nested_route()
}

const NAME_FIELD_NAME: &str = "name";

#[component]
fn CreateOrgPage() -> impl IntoView {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <form class=FORM_CLASS>
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Create an Organization"</p>
          <p class="max-w-prose whitespace-pre-line">{ ORG_DESCRIPTION }</p>
        </div>
      </GridRowFull>

      <GridRowFull>
        <div class="h-0 border-t-[1.5px] border-base-6 w-full" />
      </GridRowFull>

      <GridRow>
        <GridRowLabel
          title="Org name"
          desc="Think of it like a username."
        />

        <div class="flex flex-col gap-1">
          <label class="input-field">
            <BuildingOffice2HeroIcon {..} class="size-6 shrink-0" />
            <input
              class="w-full py-2 focus-visible:outline-none"
              type="text" autofocus=true required
              placeholder="Org Name" name=NAME_FIELD_NAME

              hx-get="/org/create_org/validate"
              hx-target="#name-hint"
              hx-swap="innerHTML transition:true"
              hx-trigger="input throttle:0.25s"
            />
          </label>
          <div id="name-hint" class="contents" />
        </div>
      </GridRow>

      <GridRow>
        <div />
        <div class="flex flex-col gap-4">
          <label>
            <input type="submit" class="hidden" />
            <button class="btn btn-primary w-full max-w-80 justify-between">
              <div class="size-4" />
              "Create Org"
              <LoadingCircle {..}
                class="size-4 transition-opacity htmx-indicator"
              />
            </button>
          </label>
          <div id="form-result" class="contents" />
        </div>
      </GridRow>
    </form>
  }
}

#[component]
fn OrgNameValidation() -> impl IntoView {
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
