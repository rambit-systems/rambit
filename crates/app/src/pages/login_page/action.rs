use std::{collections::HashMap, time::Duration};

use axum::{Form, response::IntoResponse};
use maud::html;
use models::{AuthUser, EmailAddress, UserSubmittedAuthCredentials};

use crate::{
  components::{
    form_result::{form_acceptance, form_rejection},
    scripts::redirect_script,
  },
  ctx::{Ctx, ResponseSeed},
  form_feedback_text::*,
  hooks::OrgUrlHook,
};

mod extract;

pub(super) async fn login_action(
  ResponseSeed(ctx, resp): ResponseSeed,
  Form(map): Form<HashMap<String, String>>,
) -> impl IntoResponse {
  let (email, creds) = match self::extract::extract_fields(map) {
    Ok(r) => r,
    Err(message) => {
      let document = form_rejection(message);
      return resp.into_stream(document).into_response();
    }
  };

  let result = form_action(ctx, email.clone(), creds.clone()).await;
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

  resp.into_stream(feedback).into_response()
}

async fn form_action(
  ctx: Ctx,
  email: EmailAddress,
  creds: UserSubmittedAuthCredentials,
) -> Result<AuthUser, &'static str> {
  let mut auth_session = ctx.auth_session();

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
