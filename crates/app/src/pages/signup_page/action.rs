use std::{collections::HashMap, time::Duration};

use axum::response::IntoResponse;
use domain::create::CreateUserError;
use maud::html;
use models::{AuthUser, EmailAddress, HumanName, UserSubmittedAuthCredentials};

use crate::{
  components::{
    form_result::{form_acceptance, form_rejection},
    scripts::redirect_script,
  },
  ctx::{Ctx, ResponseSeed},
  extractors::{FromFormMap, FromValidatedForm, ValidatedForm},
  form_feedback_text::*,
  hooks::OrgUrlHook,
};

pub(super) struct SignupParams {
  name:  HumanName,
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
}

impl FromValidatedForm for SignupParams {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String> {
    Ok(Self {
      name:  FromFormMap::from_form_map(map)?,
      email: FromFormMap::from_form_map(map)?,
      creds: FromFormMap::from_form_map(map)?,
    })
  }
}

pub(super) async fn signup_action(
  ResponseSeed(ctx, resp): ResponseSeed,
  ValidatedForm(params): ValidatedForm<SignupParams>,
) -> impl IntoResponse {
  let SignupParams { name, email, creds } = params;

  // we're running this inline (not suspended) because we need it to finish and
  // set the auth cookie before the body starts. If we run this after the body
  // starts, the cookie header can't be modified, and the user isn't logged in.
  let result = form_action(ctx, name, email, creds).await;

  let feedback = match result {
    Ok(au) => {
      let target = OrgUrlHook::new(au.active_org()).dashboard_url();
      html! {
        (form_acceptance(SUCCESS_MESSAGE))
        (redirect_script(target, Some(Duration::from_millis(500))))
      }
    }
    Err(e) => form_rejection(e),
  };

  resp.into_stream(feedback)
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
