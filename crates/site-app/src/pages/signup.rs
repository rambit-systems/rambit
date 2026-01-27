use std::collections::HashMap;

use auth_domain::AuthSession;
use axum::Form;
use domain::{create::CreateUserError, DomainService};
use futures::FutureExt;
use grid_state::AppState;
use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::{
  EmailAddress, HumanName, HumanNameError, UserSubmittedAuthCredentials,
};

use crate::{
  components::{
    form_acceptance, form_layout::*, form_rejection, EnvelopeHeroIcon,
    LoadingCircle, LockClosedHeroIcon, UserHeroIcon,
  },
  form_feedback_text::*,
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
  let form_data = leptos_router::hooks::use_query_map().get();

  let Some(name) = form_data.get(NAME_FIELD_NAME) else {
    return form_rejection(const_format::formatcp!(
      "The signup request did not contain the \"{NAME_FIELD_NAME}\" field :/"
    ))
    .into_any();
  };
  if name.is_empty() {
    return form_rejection(EMPTY_NAME_MESSAGE).into_any();
  }
  let name = match HumanName::try_new(name) {
    Ok(name) => name,
    Err(HumanNameError::Empty) => unreachable!(),
    Err(HumanNameError::TooLong) => {
      return form_rejection(NAME_TOO_LONG_MESSAGE).into_any();
    }
  };

  let Some(email) = form_data.get(EMAIL_FIELD_NAME) else {
    return form_rejection(const_format::formatcp!(
      "The signup request did not contain the \"{EMAIL_FIELD_NAME}\" field :/"
    ))
    .into_any();
  };
  if email.is_empty() {
    return form_rejection(EMPTY_EMAIL_MESSAGE).into_any();
  }
  let email = match EmailAddress::try_new(email) {
    Ok(email) => email,
    Err(models::EmailAddressError::InvalidEmail) => {
      return form_rejection(MALFORMED_EMAIL_MESSAGE).into_any();
    }
    Err(models::EmailAddressError::TooLong) => {
      return form_rejection(EMAIL_TOO_LONG_MESSAGE).into_any();
    }
  };

  let Some(password) = form_data.get(PASSWORD_FIELD_NAME) else {
    return form_rejection(const_format::formatcp!(
      "The signup request did not contain the \"{PASSWORD_FIELD_NAME}\" field \
       :/"
    ))
    .into_any();
  };
  if password.is_empty() {
    return form_rejection(EMPTY_PASSWORD_MESSAGE).into_any();
  }

  let Some(confirm_password) = form_data.get(CONFIRM_FIELD_NAME) else {
    return form_rejection(const_format::formatcp!(
      "The signup request did not contain the \"{CONFIRM_FIELD_NAME}\" field \
       :/"
    ))
    .into_any();
  };
  if confirm_password != password {
    return form_rejection(PASSWORD_CONFIRM_MISMATCH_MESSAGE).into_any();
  }

  let creds = UserSubmittedAuthCredentials::Password {
    password: password.to_owned(),
  };

  let auth_session = expect_context::<AuthSession>();
  let state = expect_context::<AppState>();
  let domain = state.domain.clone();

  let future = form_action(
    domain.clone(),
    auth_session.clone(),
    name.clone(),
    email.clone(),
    creds.clone(),
  );

  view! {
    <Await future=future blocking=true let:data>
      { match data {
        Ok(t) => form_acceptance(t).into_any(),
        Err(t) => form_rejection(t).into_any(),
      }}
    </Await>
  }
  .into_any()
}

async fn form_action(
  domain: DomainService,
  mut auth_session: AuthSession,
  name: HumanName,
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
) -> Result<String, String> {
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

  Ok(SUCCESS_MESSAGE.to_string())
}
