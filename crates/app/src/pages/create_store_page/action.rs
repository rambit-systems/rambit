//! Form action for the create-store page.

use std::{collections::HashMap, time::Duration};

use axum::response::IntoResponse;
use maud::html;
use models::{
  EntityName, Org, R2StorageCredentials, RecordId, Store, StoreConfiguration,
  StorageCredentials,
};

use super::credentials_input::{
  ACCESS_KEY_FIELD_NAME, BUCKET_FIELD_NAME, ENDPOINT_FIELD_NAME,
  SECRET_ACCESS_KEY_FIELD_NAME,
};
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

pub(super) struct CreateStoreParams {
  name:             EntityName,
  access_key:       String,
  secret_access_key: String,
  bucket:           String,
  endpoint:         String,
}

impl FromValidatedForm for CreateStoreParams {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String> {
    Ok(Self {
      name:             FromFormMap::from_form_map(map)?,
      access_key:       required_field(map, ACCESS_KEY_FIELD_NAME)?,
      secret_access_key: required_field(map, SECRET_ACCESS_KEY_FIELD_NAME)?,
      bucket:           required_field(map, BUCKET_FIELD_NAME)?,
      endpoint:         required_field(map, ENDPOINT_FIELD_NAME)?,
    })
  }
}

fn required_field(
  map: &HashMap<String, String>,
  field: &str,
) -> Result<String, String> {
  match map.get(field) {
    Some(v) if !v.is_empty() => Ok(v.clone()),
    Some(_) => Err(format!("The \"{field}\" field must not be empty :/")),
    None => Err(format!(
      "The action request did not contain the \"{field}\" field :/"
    )),
  }
}

pub(super) async fn create_store_action(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
  ValidatedForm(params): ValidatedForm<CreateStoreParams>,
) -> impl IntoResponse {
  let CreateStoreParams {
    name,
    access_key,
    secret_access_key,
    bucket,
    endpoint,
  } = params;
  let org_id = ctx.requested_org_url_hook().id();

  let result =
    form_action(ctx, name, access_key, secret_access_key, bucket, endpoint, org_id).await;

  let feedback = match result {
    Ok(_) => {
      let target = OrgUrlHook::new(org_id).dashboard_url();
      html! {
        (form_acceptance(SUCCESS_MESSAGE))
        (redirect_script(target, Some(Duration::from_millis(500))))
      }
    }
    Err(e) => {
      tracing::error!("failed to execute create-store action: {e:?}");
      form_rejection(INTERNAL_ERROR_MESSAGE)
    }
  };

  resp.into_stream(feedback)
}

async fn form_action(
  ctx: Ctx<RequireRequestedOrg>,
  name: EntityName,
  access_key: String,
  secret_access_key: String,
  bucket: String,
  endpoint: String,
  org: RecordId<Org>,
) -> Result<RecordId<Store>, miette::Report> {
  let auth_user = ctx.auth_user();
  if !auth_user.belongs_to_org(org) {
    miette::bail!("{UNAUTHORIZED_MESSAGE}");
  }

  let domain_service = ctx.state().domain.clone();

  let store = Store {
    id:          RecordId::new(),
    org,
    name,
    credentials: StorageCredentials::R2(R2StorageCredentials::Default {
      access_key,
      secret_access_key,
      endpoint,
      bucket,
    }),
    config:      StoreConfiguration {},
  };

  let store_id = domain_service
    .create_store(&store)
    .await
    .inspect_err(|e| {
      tracing::error!("failed to create store: {e:#?}");
    })?;

  Ok(store_id)
}
