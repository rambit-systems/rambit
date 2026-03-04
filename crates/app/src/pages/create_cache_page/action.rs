//! Form action for the create-cache page.

use std::{collections::HashMap, time::Duration};

use axum::response::IntoResponse;
use maud::html;
use models::{Cache, EntityName, Org, RecordId, Visibility};

use crate::{
  components::{
    form_result::{form_acceptance, form_rejection},
    scripts::redirect_script,
  },
  ctx::{Ctx, RequireRequestedOrg, ResponseSeed},
  extractors::{FromFormMap, FromValidatedForm, ValidatedForm},
  form_feedback_text::*,
  hooks::OrgUrlHook,
};

pub(super) struct CreateCacheParams {
  name:       EntityName,
  visibility: Visibility,
}

impl FromValidatedForm for CreateCacheParams {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String> {
    Ok(Self {
      name:       FromFormMap::from_form_map(map)?,
      visibility: FromFormMap::from_form_map(map)?,
    })
  }
}

pub(super) async fn create_cache_action(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
  ValidatedForm(params): ValidatedForm<CreateCacheParams>,
) -> impl IntoResponse {
  let CreateCacheParams { name, visibility } = params;
  let org_id = ctx.requested_org_url_hook().id();

  let result = form_action(ctx, name, visibility, org_id).await;

  let feedback = match result {
    Ok(_cache_id) => {
      // Navigate back to the org dashboard after a short delay.
      let target = OrgUrlHook::new(org_id).dashboard_url();
      html! {
        (form_acceptance(SUCCESS_MESSAGE))
        (redirect_script(target, Some(Duration::from_millis(500))))
      }
    }
    Err(e) => {
      tracing::error!("failed to execute create-cache action: {e:?}");
      form_rejection(INTERNAL_ERROR_MESSAGE)
    }
  };

  resp.into_stream(feedback)
}

async fn form_action(
  ctx: Ctx<RequireRequestedOrg>,
  name: EntityName,
  visibility: Visibility,
  org: RecordId<Org>,
) -> Result<RecordId<Cache>, miette::Report> {
  let auth_user = ctx.auth_user();
  if !auth_user.belongs_to_org(org) {
    miette::bail!("{UNAUTHORIZED_MESSAGE}");
  }

  let domain_service = ctx.state().domain.clone();

  let cache = Cache {
    id: RecordId::new(),
    org,
    name,
    visibility,
  };

  let cache_id = domain_service
    .create_cache(&cache)
    .await
    .inspect_err(|e| {
      tracing::error!("failed to create cache: {e:#?}");
    })?;

  Ok(cache_id)
}
