mod validate;

use std::time::Duration;

use domain::DomainService;
use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::{AuthUser, EntityName, Org, RecordId};

use self::validate::OrgNameValidation;
use crate::{
  components::{
    form_acceptance, form_layout::*, form_rejection, BuildingOffice2HeroIcon,
    LoadingCircle,
  },
  form_feedback_text::*,
  hooks::OrgHook,
  navigation::RedirectScript,
  pages::protect,
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
    <Route path=path!("/org/create_org/action") view=protect(CreateOrgFormAction) />
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
    <form
      class=FORM_CLASS
      hx-get="/org/create_org/action"
      hx-target="#form-result"
      hx-swap="innerHTML transition:true"
    >
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
              hx-indicator="next svg"
            />
            <LoadingCircle {..}
              class="size-6 transition-opacity htmx-indicator"
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
fn CreateOrgFormAction() -> impl IntoView {
  let form_data = leptos_router::hooks::use_query_map().get();
  let Some(name) = form_data.get(NAME_FIELD_NAME) else {
    return form_rejection(const_format::formatcp!(
      "The validation request did not contain the \"{NAME_FIELD_NAME}\" field \
       :/"
    ))
    .into_any();
  };

  if name.is_empty() {
    return form_rejection(EMPTY_NAME_MESSAGE).into_any();
  }
  let name = EntityName::new(name);

  let future = form_action(name.clone());
  let future_response = move |data: &Result<RecordId<Org>, String>| match data {
    Ok(new_org) => {
      let new_org = *new_org;
      let org_hook = OrgHook::new(move || new_org);
      let target = org_hook.dashboard_url()();

      view! {
        { form_acceptance(SUCCESS_MESSAGE) }
        <RedirectScript
          target=target
          delay={Duration::from_secs_f32(0.5)}
        />
      }
      .into_any()
    }
    Err(t) => form_rejection(t).into_any(),
  };

  view! {
    <Await future=future blocking=true let:data>
      { future_response(data) }
    </Await>
  }
  .into_any()
}

async fn form_action(name: EntityName) -> Result<RecordId<Org>, String> {
  let auth_user = use_context::<AuthUser>().ok_or(UNAUTHENTICATED_MESSAGE)?;
  let domain_service: DomainService = expect_context();

  let org = domain_service
    .create_named_org_with_user(auth_user.id, name)
    .await
    .map_err(|e| {
      tracing::error!("failed to create named org with user: {e:#?}");
      INTERNAL_ERROR_MESSAGE
    })?;

  Ok(org.id)
}
