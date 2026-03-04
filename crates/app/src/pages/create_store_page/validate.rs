//! Store name availability validation endpoint (per-org).

use std::collections::HashMap;

use axum::{extract::Query, response::IntoResponse};
use domain::db::DatabaseError;
use maud::html;
use models::{EntityName, Org, RecordId};

use super::NAME_FIELD_NAME;
use crate::{
  components::form_result::*,
  ctx::{Ctx, RequireRequestedOrg, ResponseSeed},
};

pub(super) async fn validate_store_name(
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
  Query(query_map): Query<HashMap<String, String>>,
) -> impl IntoResponse {
  let Some(name) = query_map.get(NAME_FIELD_NAME) else {
    return resp.into_stream(hint_critical(const_format::formatcp!(
      "The validation request did not contain the \"{NAME_FIELD_NAME}\" field \
       :/"
    )));
  };

  if name.is_empty() {
    return resp.into_stream(html! {});
  }

  let sanitized_name = EntityName::new(name.clone());

  let unsanitized = sanitized_name.as_ref() != name;
  let sanitization_hint = if unsanitized {
    hint_warning(format!(
      "This name will be converted to \"{sanitized_name}\""
    ))
  } else {
    Default::default()
  };

  let org_id = ctx.requested_org_url_hook().id();

  let availability_hint_suspense = ctx.suspend(
    move |ctx| async move {
      match store_name_is_available(ctx, org_id, sanitized_name).await {
        Ok(true) => hint_neutral("This name is available."),
        Ok(false) => hint_critical("This name is not available."),
        Err(_) => hint_critical("Unable to check store name availability."),
      }
    },
    hint_neutral("Checking availability..."),
  );

  resp.into_stream(html! {
    div class="flex flex-col" {
      (sanitization_hint)
      (availability_hint_suspense)
    }
  })
}

async fn store_name_is_available(
  ctx: Ctx<RequireRequestedOrg>,
  org: RecordId<Org>,
  name: EntityName,
) -> Result<bool, DatabaseError> {
  let meta = ctx.state().domain.meta();
  let is_available = meta
    .fetch_store_by_org_and_name(org, name)
    .await
    .inspect_err(|e| {
      tracing::error!("failed to fetch store by org and name: {e}");
    })?
    .is_none();

  Ok(is_available)
}
