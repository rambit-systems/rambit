use std::collections::HashMap;

use auth_domain::AuthSession;
use axum::extract::Form;
use futures::FutureExt;
use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::{EmailAddress, UserSubmittedAuthCredentials};

use crate::components::*;

#[component(transparent)]
pub fn LoginPageRoutes() -> impl MatchNestedRoutes + Clone {
  view! {
    <Route path=path!("/auth/login") view=LoginPage />
    <Route path=path!("/auth/login/action") view=LoginFormAction />
  }
  .into_inner()
  .into_any_nested_route()
}

const EMAIL_FIELD_NAME: &str = "email";
const PASSWORD_FIELD_NAME: &str = "password";

#[component]
fn LoginPage() -> impl IntoView {
  view! {
    <div class="p-8 self-stretch sm:self-center w-auto elevation-flat">
      <form
        class="max-w-80 flex flex-col gap-6"
        hx-post="/auth/login/action"
        hx-target="#form-result"
        hx-swap="innerHTML transition:true"
      >
        <p class="title">"Login"</p>
        <p class="max-w-prose">
          "Welcome back to the most satisfying part of your CI/CD pipeline."
        </p>

        <div class="flex flex-col gap-4">
          // email
          <label class="flex flex-col gap-1">
            <p class="text-base-11">"Email Address"</p>
            <div class="input-field">
              <EnvelopeHeroIcon {..} class="size-6 shrink-0" />
              <input
                class="w-full py-2 focus-visible:outline-none" required
                type="email" autofocus=true placeholder="" name=EMAIL_FIELD_NAME
              />
            </div>
          </label>

          // password
          <label class="flex flex-col gap-1">
            <p class="text-base-11">"Password"</p>
            <div class="input-field">
              <LockClosedHeroIcon {..} class="size-6 shrink-0" />
              <input
                class="w-full py-2 focus-visible:outline-none" required
                type="password" placeholder="" name=PASSWORD_FIELD_NAME
              />
            </div>
          </label>
        </div>

        // submit
        <div class="flex flex-col gap-4">
          <label>
            <input type="submit" class="hidden" />
            <button class="btn btn-primary w-full justify-between">
              <div class="size-4" />
              "Log in to Rambit"
              <LoadingCircle {..}
                class="size-4 transition-opacity htmx-indicator"
              />
            </button>
          </label>
          <div id="form-result" class="contents" />
        </div>
      </form>
    </div>
  }
}

const INTERNAL_ERROR_MESSAGE: &str = "Oops! This is embarrasing... looks like \
                                      we're having trouble logging you in :/";
const SUCCESS_MESSAGE: &str = "Success! Redirecting :)";
const UNAUTHORIZED_MESSAGE: &str =
  "Oops! Looks like those aren't the right credentials :/";
const NOT_FORM_MESSAGE: &str = "The login request did not contain form data :/";
const MISSING_EMAIL_MESSAGE: &str = const_format::formatcp!(
  "The login request did not contain the \"{EMAIL_FIELD_NAME}\" field :/"
);
const EMPTY_EMAIL_MESSAGE: &str =
  "Looks like you forgot to put in your email :/";
const MALFORMED_EMAIL_MESSAGE: &str =
  "Sorry but that doesn't really look like an email address :/";
const EMAIL_TOO_LONG_MESSAGE: &str = "Sorry but that email is too long :/";
const MISSING_PASSWORD_MESSAGE: &str =
  "The login request did not contain the \"{PASSWORD_FIELD_NAME}\" field :/";
const EMPTY_PASSWORD_MESSAGE: &str =
  "Looks like you forgot to put in your password :/";

#[component]
fn LoginFormAction() -> impl IntoView {
  let form_data = use_context::<Form<HashMap<String, String>>>();
  let Some(form_data) = form_data else {
    return form_rejection(NOT_FORM_MESSAGE).into_any();
  };

  let Some(email) = form_data.get(EMAIL_FIELD_NAME) else {
    return form_rejection(MISSING_EMAIL_MESSAGE).into_any();
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
    return form_rejection(MISSING_PASSWORD_MESSAGE).into_any();
  };
  if password.is_empty() {
    return form_rejection(EMPTY_PASSWORD_MESSAGE).into_any();
  }
  let creds = UserSubmittedAuthCredentials::Password {
    password: password.to_owned(),
  };

  let auth_session = expect_context::<AuthSession>();

  let suspend = move || {
    Suspend::new(
      form_action(auth_session.clone(), email.clone(), creds.clone()).map(
        |r| match r {
          Ok(t) => form_acceptance(t).into_any(),
          Err(t) => form_rejection(t).into_any(),
        },
      ),
    )
  };

  view! {
    <Suspense fallback=|| "Loading...">
      { suspend }
    </Suspense>
  }
  .into_any()
}

async fn form_action(
  mut auth_session: AuthSession,
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
) -> Result<&'static str, &'static str> {
  match auth_session.authenticate((email.clone(), creds)).await {
    Ok(Some(user)) => match auth_session.login(&user).await {
      Ok(()) => Ok(SUCCESS_MESSAGE),
      Err(err) => {
        tracing::error!(
          "encountered error while authenticating user ({email}): {err:?}"
        );
        Err(INTERNAL_ERROR_MESSAGE)
      }
    },
    Ok(None) => Err(UNAUTHORIZED_MESSAGE),
    Err(err) => {
      tracing::error!(
        "encountered error while logging in user ({email}): {err:?}"
      );
      Err(INTERNAL_ERROR_MESSAGE)
    }
  }
}

fn form_rejection(text: impl AsRef<str>) -> impl IntoView {
  view! {
    <p id="form-rejection" class="text-critical-11">
      { text.as_ref() }
    </p>
  }
}

fn form_acceptance(text: impl AsRef<str>) -> impl IntoView {
  view! {
    <p id="form-acceptance" class="text-base-12">
      { text.as_ref() }
    </p>
  }
}
