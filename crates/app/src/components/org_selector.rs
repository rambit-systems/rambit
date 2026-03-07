use std::collections::HashMap;

use axum::{
  Router,
  extract::Path,
  http::{HeaderValue, StatusCode, header},
  response::IntoResponse,
  routing::get,
};
use const_format::concatcp;
use domain::UpdateActiveOrgError;
use grid_state::AppState;
use maud::{Markup, html};
use models::{Org, RecordId};

use crate::{
  APP_PREFIX,
  components::{
    form_result::form_rejection, icons::*, text_data::org_descriptor,
  },
  ctx::{Ctx, RequireAuth, ResponseSeed},
  form_feedback_text::INTERNAL_ERROR_MESSAGE,
  hooks::OrgUrlHook,
};

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/action/{id}", get(org_selector_action))
}

pub fn org_selector(ctx: Ctx<RequireAuth>) -> Markup {
  let active_org_id = ctx.active_org_url_hook().id();

  const BUTTON_CLASS: &str = "transition-colors hover:bg-base-3 \
                              active:bg-base-4 button-squish cursor-pointer \
                              px-2 py-1 rounded flex flex-col gap-0.5 text-sm \
                              leading-none gap-0";
  const BUTTON_ANCHOR_CLASS: &str = "[anchor-name:--org-selector-anchor]";
  const POPOVER_CLASS: &str =
    "min-w-56 elevation-lv1 p-2 rounded transition transition-discrete \
     duration-200 opacity-0 scale-97 [&:popover-open]:opacity-100 \
     [&:popover-open]:scale-100 [@starting-style]:[&:popover-open]:opacity-0 \
     [@starting-style]:[&:popover-open]:scale-97";
  const POPOVER_ANCHOR_CLASS: &str = "[position-anchor:--org-selector-anchor] \
                                      top-[anchor(bottom)] \
                                      left-[anchor(right)] -translate-x-full \
                                      translate-y-[calc(var(--spacing)*4)]";

  let menu_suspense = ctx.suspend(org_selector_menu, html! {
    div class="py-3 w-full flex flex-row gap-2 items-center justify-center text-base-11" {
      "Loading"
      div class="size-4" { (loading_circle()) }
    }
  });

  html! {
    button
      class=([BUTTON_CLASS, BUTTON_ANCHOR_CLASS].join(" "))
      popovertarget="org-selector-popover"
    {
      p class="text-base/[1] text-base-12" {
        (org_descriptor(ctx, active_org_id))
      }
      div class="flex flex-row items-end gap-0.5" {
        div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (chevron_down_hero_icon()) }
        p { "Switch Orgs" }
      }
    }

    div
      popover
      id="org-selector-popover"
      class=([POPOVER_CLASS, POPOVER_ANCHOR_CLASS].join(" "))
    {
      (menu_suspense)
    }
  }
}

async fn org_selector_menu(ctx: Ctx<RequireAuth>) -> Markup {
  let auth_user = ctx.auth_user();
  let active_org = auth_user.active_org();
  let org_rows = auth_user
    .iter_orgs()
    .map(|o| org_row(ctx.clone(), o, o == active_org))
    .collect::<Vec<_>>();

  html! {
    div class="flex flex-col gap-1 leading-none" {
      @ for org_row in org_rows {
        (org_row)
      }
      div class="p-1" {
        div class="h-0 border-t-2 border-base-6/75" {}
      }
      (extra_rows(ctx))
    }
  }
}

fn org_row(
  ctx: Ctx<RequireAuth>,
  org_id: RecordId<Org>,
  active: bool,
) -> Markup {
  let icon_element = html! {
    div class="size-5 shrink-0 stroke-product-11 stroke-[2.0]" {
      @if active {
        (check_hero_icon())
      }
    }
  };
  let action_href = format!("{APP_PREFIX}/org_selector/action/{org_id}");

  let class = format!(
    "rounded p-2 flex flex-row gap-2 items-center transition-colors \
     text-base-12 {}",
    if active {
      "font-bold"
    } else {
      "cursor-pointer btn-link-secondary"
    }
  );

  html! {
    a href=(action_href) class=(class) {
      (icon_element)
      span class="text-ellipsis" {
        (org_descriptor(ctx, org_id))
      }
    }
  }
}

fn extra_rows(ctx: Ctx<RequireAuth>) -> Markup {
  const CLASS: &str = "btn-link btn-link-secondary btn-link-tight";
  const ICON_CLASS: &str = "size-5 shrink-0 stroke-base-11 stroke-[2.0]";

  let active = ctx.active_org_url_hook();
  let active_org_settings_url = active.settings_url();
  const CREATE_ORG_URL: &str = concatcp!(APP_PREFIX, "/org/create_org");

  html! {
    a href=(active_org_settings_url) class=(CLASS) {
      div class=(ICON_CLASS) { (cog_6_tooth_hero_icon()) }
      span class="text-ellipsis" {
        "Org Settings"
      }
    }
    a href=(CREATE_ORG_URL) class=(CLASS) {
      div class=(ICON_CLASS) { (plus_hero_icon()) }
      span class="text-ellipsis" {
        "Create Org"
      }
    }
  }
}

// This handler is called not from HTMX, but is hit with a regular <a> tag. It
// navigates the user by redirecting them with a status redirect and location
// header, after the mutation is complete.
async fn org_selector_action(
  ResponseSeed(ctx, resp): ResponseSeed<RequireAuth>,
  Path(path_map): Path<HashMap<String, String>>,
) -> impl IntoResponse {
  let requested_org_param =
    path_map.get("id").expect("missing action id path param");

  // fail if ID can't be parsed
  let Ok(requested_org) = requested_org_param.parse::<RecordId<_>>() else {
    return resp
      .into_stream(form_rejection("Could not parse requested org ID"))
      .into_response();
  };

  let result = action(ctx, requested_org).await;
  match result {
    Ok(new_org) => {
      let new_org_url_hook = OrgUrlHook::new(new_org);
      let target = new_org_url_hook.dashboard_url();

      let mut response = resp.into_stream(html! {}).into_response();
      let headers = response.headers_mut();
      headers.insert(
        header::LOCATION,
        HeaderValue::from_str(&target)
          .expect("failed to encode target URL as LOCATION header value"),
      );
      *response.status_mut() = StatusCode::SEE_OTHER;
      response
    }
    Err(message) => resp.into_stream(form_rejection(message)).into_response(),
  }
}

async fn action(
  ctx: Ctx<RequireAuth>,
  requested_org: RecordId<Org>,
) -> Result<RecordId<Org>, &'static str> {
  let domain_service = ctx.state().domain.clone();

  domain_service
    .switch_active_org(ctx.auth_user().id, requested_org)
    .await
    .map_err(|e| match e {
      UpdateActiveOrgError::InvalidOrg(_) => "Could not switch to this org",
      e => {
        tracing::error!("failed to fetch org: {e}");
        INTERNAL_ERROR_MESSAGE
      }
    })
}
