use std::collections::HashMap;

use axum::{extract::Path, response::IntoResponse};
use maud::{Markup, html};
use models::{Entry, RecordId};

use crate::{
  components::{
    form_result::form_rejection, icons::loading_circle,
    scripts::redirect_script,
  },
  ctx::{RequireRequestedOrg, ResponseSeed},
  hooks::OrgUrlHook,
  resources::fetch_entry,
};

pub fn action_tile(
  org_url_hook: &OrgUrlHook,
  entry_id: RecordId<Entry>,
) -> Markup {
  let delete_url = org_url_hook.entry_delete_url(entry_id);

  html! {
    div class="md:w-64 p-6 elevation-flat flex flex-col gap-4" {
      p class="subtitle" { "Actions" }
      div class="flex flex-col gap-2" {
        form
          hx-post=(delete_url)
          hx-target="#delete-result"
        {
          button
            type="submit"
            class="btn btn-critical w-full justify-between relative overflow-hidden"
          {
            div class="size-4" {}
            "Delete Entry"
            div class="size-4 transition-opacity htmx-indicator" {
              (loading_circle())
            }
          }
        }
        div id="delete-result" class="contents" {}
      }
    }
  }
}

pub async fn delete_entry_action(
  Path(params): Path<HashMap<String, String>>,
  ResponseSeed(ctx, resp): ResponseSeed<RequireRequestedOrg>,
) -> impl IntoResponse {
  use std::str::FromStr;

  let entry_id = match params
    .get("entry")
    .and_then(|s| RecordId::<Entry>::from_str(s).ok())
  {
    Some(id) => id,
    None => {
      return resp.into_stream(form_rejection("Invalid entry ID."));
    }
  };

  let org_id = ctx.requested_org_url_hook().id();
  let auth_ctx = ctx.clone().into_require_auth();

  let entry = match auth_ctx.fetch_cached(fetch_entry, entry_id).await.as_ref()
  {
    Ok(Some(e)) => e.clone(),
    Ok(None) => {
      return resp.into_stream(form_rejection("Entry not found."));
    }
    Err(_) => {
      return resp.into_stream(form_rejection("Failed to fetch entry."));
    }
  };

  if entry.org != org_id {
    return resp.into_stream(form_rejection("Unauthorized."));
  }

  match ctx.state().domain.delete_entry(entry_id).await {
    Ok(_) => {
      let dashboard_url = OrgUrlHook::new(org_id).dashboard_url();
      resp.into_stream(redirect_script(dashboard_url, None))
    }
    Err(e) => {
      tracing::error!("failed to delete entry {entry_id}: {e:?}");
      resp.into_stream(form_rejection("Failed to delete entry."))
    }
  }
}
