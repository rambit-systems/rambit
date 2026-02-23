mod extract;

use std::{collections::HashMap, time::Duration};

use axum::{Form, response::IntoResponse};
use domain::create::CreateUserError;
use maud::html;
use models::{AuthUser, EmailAddress, HumanName, UserSubmittedAuthCredentials};

use crate::{
  components::{
    form_result::{form_acceptance, form_rejection},
    scripts::redirect_script,
  },
  ctx::{Ctx, ResponseSeed},
  form_feedback_text::*,
  hooks::OrgUrlHook,
};

pub(super) async fn signup_action(
  ResponseSeed(ctx, resp): ResponseSeed,
  Form(map): Form<HashMap<String, String>>,
) -> impl IntoResponse {
  let (name, email, creds) = match self::extract::extract_fields(map) {
    Ok(r) => r,
    Err(message) => {
      let document = form_rejection(message);
      return resp.into_stream(document).into_response();
    }
  };

  // we're running this inline (not suspended) because we need it to finish and
  // set the auth cookie before the body starts. If we run this after the body
  // starts, the cookie header can't be modified, and the user isn't logged in.
  let action_result = match form_action(ctx, name, email, creds).await {
    Ok(au) => {
      let target = OrgUrlHook::new(au.active_org()).dashboard_url();
      html! {
        (form_acceptance(SUCCESS_MESSAGE))
        (redirect_script(target, Some(Duration::from_millis(500))))
      }
    }
    Err(e) => html! { (form_rejection(e)) },
  };

  resp.into_stream(html! { (action_result) }).into_response()
}

async fn form_action(
  ctx: Ctx,
  name: HumanName,
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
) -> Result<AuthUser, &'static str> {
  let mut auth_session = ctx.auth_session();
  let domain = ctx.state().domain.clone();

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
