use std::{collections::HashMap, time::Duration};

use axum::response::IntoResponse;
use maud::html;
use models::{EntityName, Org, RecordId};

use crate::{
  components::{
    form_result::{form_acceptance, form_rejection},
    scripts::redirect_script,
  },
  ctx::{Ctx, RequireAuth, ResponseSeed},
  extractors::{FromFormMap, FromValidatedForm, ValidatedForm},
  form_feedback_text::*,
  hooks::OrgUrlHook,
};

pub(super) struct CreateOrgParams {
  name: EntityName,
}

impl FromValidatedForm for CreateOrgParams {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String> {
    Ok(Self {
      name: FromFormMap::from_form_map(map)?,
    })
  }
}

pub(super) async fn signup_action(
  ResponseSeed(ctx, resp): ResponseSeed<RequireAuth>,
  ValidatedForm(params): ValidatedForm<CreateOrgParams>,
) -> impl IntoResponse {
  let CreateOrgParams { name } = params;

  // we're running this inline (not suspended) because we need it to finish and
  // set the auth cookie before the body starts. If we run this after the body
  // starts, the cookie header can't be modified, and the active org isn't
  // switched.
  let result = form_action(ctx, name).await;

  let feedback = match result {
    Ok(new_org) => {
      let target = OrgUrlHook::new(new_org).dashboard_url();
      html! {
        (form_acceptance(SUCCESS_MESSAGE))
        (redirect_script(target, Some(Duration::from_millis(500))))
      }
    }
    Err(e) => form_rejection(INTERNAL_ERROR_MESSAGE),
  };

  resp.into_stream(feedback)
}

async fn form_action(
  ctx: Ctx<RequireAuth>,
  name: EntityName,
) -> Result<RecordId<Org>, miette::Report> {
  let domain_service = ctx.state().domain.clone();
  let auth_user = ctx.auth_user();

  let org = domain_service
    .create_named_org_with_user(auth_user.id, name)
    .await
    .inspect_err(|e| {
      tracing::error!("failed to create named org with user: {e:#?}");
    })?;

  Ok(org.id)
}
