mod extract_fields;

use std::time::Duration;

use auth_domain::AuthSession;
use domain::create::CreateUserError;
use grid_state::AppState;
use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::{AuthUser, EmailAddress, HumanName, UserSubmittedAuthCredentials};

use crate::{
  components::{
    form_acceptance, form_layout::*, form_rejection, EnvelopeHeroIcon,
    LoadingCircle, LockClosedHeroIcon, UserHeroIcon,
  },
  form_feedback_text::*,
  hooks::OrgHook,
  navigation::RedirectScript,
};

#[component(transparent)]
pub fn SignupPageRoutes() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("/auth/signup") view=SignupPage />
    <Route path=path!("/auth/signup/action") view=SignupFormAction />
  }
  .into_inner()
  .into_any_nested_route()
}

const NAME_FIELD_NAME: &str = "name";
const EMAIL_FIELD_NAME: &str = "email";
const PASSWORD_FIELD_NAME: &str = "password";
const CONFIRM_FIELD_NAME: &str = "confirm";

#[component]
fn SignupPage() -> impl IntoView {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <form
      class=FORM_CLASS
      hx-get="/auth/signup/action"
      hx-target="#form-result"
      hx-swap="innerHTML transition:true"
    >
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Sign Up"</p>
          <p class="max-w-prose">
            "Thanks so much for trying us out — we can't wait to get you up and \
            running in no time. Prepare for magical iteration cycle times."
          </p>
        </div>
      </GridRowFull>

      <GridRow>
        <GridRowLabel
          title="Your name"
          desc="What do you like to be called?"
        />

        <label class="input-field">
          <UserHeroIcon {..} class="size-6 shrink-0" />
          <input
            class="w-full py-2 focus-visible:outline-none"
            type="text" autofocus=true required
            placeholder="Full Name" name=NAME_FIELD_NAME
          />
        </label>
      </GridRow>

      <GridRow>
        <GridRowLabel
          title="Your email"
          desc="[Helpful description goes here]"
        />

        <label class="input-field">
          <EnvelopeHeroIcon {..} class="size-6 shrink-0" />
          <input
            class="w-full py-2 focus-visible:outline-none" required
            type="email" placeholder="Email Address" name=EMAIL_FIELD_NAME
          />
        </label>
      </GridRow>

      <GridRow>
        <GridRowLabel
          title="Pick a password"
          desc="[Helpful description goes here]"
        />

        <div class="flex flex-col gap-1">
          <label class="input-field">
            <LockClosedHeroIcon {..} class="size-6 shrink-0" />
            <input
              class="w-full py-2 focus-visible:outline-none" required
              type="password" placeholder="Password" name=PASSWORD_FIELD_NAME
            />
          </label>
          <label class="input-field">
            <LockClosedHeroIcon {..} class="size-6 shrink-0" />
            <input
              class="w-full py-2 focus-visible:outline-none" required
              type="password" placeholder="Confirm Password" name=CONFIRM_FIELD_NAME
            />
          </label>
        </div>
      </GridRow>

      <GridRow>
        <div />
        <div class="flex flex-col gap-4">
          <label>
            <input type="submit" class="hidden" />
            <button class="btn btn-primary w-full max-w-80 justify-between">
              <div class="size-4" />
              "Sign Up"
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
fn SignupFormAction() -> impl IntoView {
  let (name, email, creds) = match self::extract_fields::extract_fields() {
    Ok(d) => d,
    Err(rejection) => return form_rejection(rejection).into_any(),
  };

  let future = form_action(name.clone(), email.clone(), creds.clone());
  let future_response = move |data: &Result<AuthUser, String>| match data {
    Ok(auth_user) => {
      provide_context(auth_user.clone());
      let org_hook = OrgHook::new_active();
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
  name: HumanName,
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
) -> Result<AuthUser, String> {
  let mut auth_session = expect_context::<AuthSession>();
  let state = expect_context::<AppState>();
  let domain = state.domain.clone();

  let user = domain
    .user_signup(name, email.clone(), creds.clone())
    .await
    .map_err(|e| match e {
      CreateUserError::EmailAlreadyUsed(_) => EMAIL_ALREADY_USED_MESSAGE,
      CreateUserError::InternalError(err) => {
        tracing::error!(
          "encountered error while signing up user ({email}): {err:?}"
        );
        INTERNAL_ERROR_MESSAGE
      }
    })?;

  let auth_user = user.into();

  auth_session.login(&auth_user).await.map_err(|err| {
    tracing::error!(
      "encountered error while authenticating user ({email}): {err:?}"
    );
    INTERNAL_ERROR_MESSAGE
  })?;

  Ok(auth_user)
}
