use auth_domain::AuthSession;
use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::{EmailAddress, UserSubmittedAuthCredentials};

use crate::{components::*, form_feedback_text::*};

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
        hx-get="/auth/login/action"
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

#[component]
fn LoginFormAction() -> impl IntoView {
  let form_data = leptos_router::hooks::use_query_map().get();

  let Some(email) = form_data.get(EMAIL_FIELD_NAME) else {
    return form_rejection(const_format::formatcp!(
      "The login request did not contain the \"{EMAIL_FIELD_NAME}\" field :/"
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
      "The login request did not contain the \"{PASSWORD_FIELD_NAME}\" field \
       :/"
    ))
    .into_any();
  };
  if password.is_empty() {
    return form_rejection(EMPTY_PASSWORD_MESSAGE).into_any();
  }
  let creds = UserSubmittedAuthCredentials::Password {
    password: password.to_owned(),
  };

  let auth_session = expect_context::<AuthSession>();

  let future = form_action(auth_session.clone(), email.clone(), creds.clone());

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
  mut auth_session: AuthSession,
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
) -> Result<String, String> {
  let user = auth_session
    .authenticate((email.clone(), creds))
    .await
    .map_err(|err| {
      tracing::error!(
        "encountered error while logging in user ({email}): {err:?}"
      );
      INTERNAL_ERROR_MESSAGE
    })?
    .ok_or(UNAUTHORIZED_MESSAGE)?;

  auth_session.login(&user).await.map_err(|err| {
    tracing::error!(
      "encountered error while authenticating user ({email}): {err:?}"
    );
    INTERNAL_ERROR_MESSAGE
  })?;

  Ok(SUCCESS_MESSAGE.to_string())
}
