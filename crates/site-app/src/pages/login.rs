mod extract_fields;

use std::time::Duration;

use auth_domain::AuthSession;
use leptos::prelude::*;
use leptos_router::{
  any_nested_route::IntoAnyNestedRoute, components::Route, path,
};
use models::{AuthUser, EmailAddress, UserSubmittedAuthCredentials};

use crate::{
  components::*, form_feedback_text::*, hooks::OrgHook,
  navigation::RedirectScript,
};

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
  let (email, creds) = match self::extract_fields::extract_fields() {
    Ok(d) => d,
    Err(rejection) => return form_rejection(rejection).into_any(),
  };

  let future = form_action(email.clone(), creds.clone());
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
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
) -> Result<AuthUser, String> {
  let mut auth_session = expect_context::<AuthSession>();

  let auth_user = auth_session
    .authenticate((email.clone(), creds))
    .await
    .map_err(|err| {
      tracing::error!(
        "encountered error while logging in user ({email}): {err:?}"
      );
      INTERNAL_ERROR_MESSAGE
    })?
    .ok_or(UNAUTHORIZED_MESSAGE)?;

  auth_session.login(&auth_user).await.map_err(|err| {
    tracing::error!(
      "encountered error while authenticating user ({email}): {err:?}"
    );
    INTERNAL_ERROR_MESSAGE
  })?;

  Ok(auth_user)
}
