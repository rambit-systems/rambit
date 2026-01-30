mod validate;
mod visibility_selector;

use std::{str::FromStr, time::Duration};

use domain::DomainService;
use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::{AuthUser, Cache, EntityName, Org, RecordId, Visibility};

use self::{
  validate::CacheNameValidation, visibility_selector::VisibilitySelector,
};
use crate::{
  components::{
    form_acceptance, form_layout::*, form_rejection, ArchiveBoxHeroIcon,
    LoadingCircle,
  },
  form_feedback_text::*,
  hooks::OrgHook,
  navigation::RedirectScript,
  pages::protect_by_org,
};

const CACHE_DESCRIPTION: &str =
  "A cache is a container and access-control mechanism for entries, and is \
   the primary namespace through which users will consume your entries.

   A cache's name must be globally unique (across organizations), even if the \
   cache is set to private. The visibility of the cache controls whether its \
   entries are accessible outside of your organization.

   Generally cache names are on a first-come-first-served basis, but please \
   contact us if you have concerns.";

#[component(transparent)]
pub fn CreateCachePageRoutes() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("/org/:org/create_cache") view=protect_by_org(CreateCachePage) />
    <Route path=path!("/org/:org/create_cache/validate") view=protect_by_org(CacheNameValidation) />
    <Route path=path!("/org/:org/create_cache/action") view=protect_by_org(CreateCacheFormAction) />
  }
  .into_inner()
  .into_any_nested_route()
}

const NAME_FIELD_NAME: &str = "name";
const VISIBILITY_FIELD_NAME: &str = "visibility";

#[component]
fn CreateCachePage() -> impl IntoView {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  let org_hook = OrgHook::new_requested();
  let validate_endpoint = org_hook.create_cache_validation_url()();
  let action_endpoint = org_hook.create_cache_action_url()();

  view! {
    <form
      class=FORM_CLASS
      hx-get=action_endpoint
      hx-target="#form-result"
      hx-swap="innerHTML transition:true"
    >
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Create a Cache"</p>
          <p class="max-w-prose whitespace-pre-line">{ CACHE_DESCRIPTION }</p>
        </div>
      </GridRowFull>

      <GridRowFull>
        <div class="h-0 border-t-[1.5px] border-base-6 w-full" />
      </GridRowFull>

      <GridRow>
        <GridRowLabel
          title="Cache name"
          desc="Think of it like a username."
        />

        <div class="flex flex-col gap-1">
          <label class="input-field">
            <ArchiveBoxHeroIcon {..} class="size-6 shrink-0" />
            <input
              class="w-full py-2 focus-visible:outline-none"
              type="text" autofocus=true required
              placeholder="Cache Name" name=NAME_FIELD_NAME

              hx-get=validate_endpoint
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
        <GridRowLabel
          title="Visibility"
          desc="For the public good or just your team?"
        />

        <VisibilitySelector />
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

fn CreateCacheFormAction() -> impl IntoView {
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

  let Some(visibility) = form_data.get(VISIBILITY_FIELD_NAME) else {
    return form_rejection(const_format::formatcp!(
      "The validation request did not contain the \"{VISIBILITY_FIELD_NAME}\" \
       field :/"
    ))
    .into_any();
  };
  let Ok(visibility) = Visibility::from_str(&visibility) else {
    return form_rejection(const_format::formatcp!(
      "Failed to parse the value of the \"{VISIBILITY_FIELD_NAME}\" field :/"
    ))
    .into_any();
  };

  let requested_org = OrgHook::new_requested().key()();

  let future = form_action(name.clone(), visibility, requested_org);
  let future_response = move |data: &Result<RecordId<Cache>, String>| match data
  {
    Ok(_new_cache) => {
      let org_hook = OrgHook::new_requested();
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

async fn form_action(
  name: EntityName,
  visibility: Visibility,
  org: RecordId<Org>,
) -> Result<RecordId<Cache>, String> {
  let auth_user = use_context::<AuthUser>().ok_or(UNAUTHENTICATED_MESSAGE)?;
  if !auth_user.belongs_to_org(org) {
    return Err(UNAUTHORIZED_MESSAGE.into());
  }
  let domain_service: DomainService = expect_context();

  let cache = Cache {
    id: RecordId::new(),
    org,
    name,
    visibility,
  };

  let cache = domain_service.create_cache(&cache).await.map_err(|e| {
    tracing::error!("failed to create cache: {e:#?}");
    INTERNAL_ERROR_MESSAGE
  })?;

  Ok(cache)
}
