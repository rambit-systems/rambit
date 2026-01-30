mod validate;

use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};

use self::validate::OrgNameValidation;
use crate::{
  components::{form_layout::*, BuildingOffice2HeroIcon, LoadingCircle},
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
