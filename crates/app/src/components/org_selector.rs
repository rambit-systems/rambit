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
use maud::{Markup, PreEscaped, html};
use models::{AuthUser, Org, RecordId};

use crate::{
  APP_PREFIX,
  components::{form_result::form_rejection, icons::*},
  ctx::{Ctx, ResponseSeed},
  form_feedback_text::{INTERNAL_ERROR_MESSAGE, UNAUTHENTICATED_MESSAGE},
  hooks::{OrgHook, OrgUrlHook},
};

pub fn sub_router() -> Router<AppState> {
  Router::new()
    .route("/", get(org_selector_menu))
    .route("/action/{id}", get(org_selector_action))
}

pub fn org_selector(ctx: Ctx) -> Markup {
  html! {
    (remove_popover_on_click_outside())
    (org_selector_trigger(ctx))
  }
}

fn remove_popover_on_click_outside() -> Markup {
  const SCRIPT: &str = r##"
    document.addEventListener('click', function(e) {
        const popover = document.querySelector('[data-popover]');
        const trigger = document.querySelector('[hx-target="#org-selector-popover-contents"]');
    
        if (popover && !popover.contains(e.target) && !trigger.contains(e.target)) {
            popover.remove();
        }
    });
  "##;

  html! {
    script { (PreEscaped(SCRIPT)) }
  }
}

fn org_selector_trigger(ctx: Ctx) -> Markup {
  let Some(auth_state) = ctx.auth_state() else {
    return form_rejection(UNAUTHENTICATED_MESSAGE);
  };
  let auth_user = auth_state.auth_user();
  let active_org_id = auth_state.active_org_url_hook().id();

  let descriptor_suspense = ctx.suspend(
    move |ctx| async move {
      let meta = ctx.state().domain.meta();
      match meta.fetch_org_by_id(active_org_id).await {
        Ok(Some(org)) => PreEscaped(OrgHook::new(org, auth_user).descriptor()),
        Ok(None) => html! { "[unknown]" },
        Err(_) => html! { "[error]" },
      }
    },
    html! { "[loading]" },
  );

  const CLASS: &str = "transition-colors hover:bg-base-3 active:bg-base-4 \
                       cursor-pointer px-2 py-1 rounded flex flex-col gap-0.5 \
                       text-sm leading-none items-end gap-0 relative";
  const ORG_SELECTOR_URL: &str = concatcp!(APP_PREFIX, "/org_selector");

  html! {
    div
      class=(CLASS)
      hx-get=(ORG_SELECTOR_URL)
      hx-target="#org-selector-popover-contents"
    {
      p class="text-base/[1] text-base-12" {
        (descriptor_suspense)
      }
      div class="flex flex-row items-end gap-0.5" {
        div class="size-4 shrink-0 stroke-base-11 stroke-[2.0]" { (chevron_down_hero_icon()) }
        p { "Switch Orgs" }
      }

      div id="org-selector-popover-contents" class="contents" {}
    }
  }
}

async fn org_selector_menu(
  ResponseSeed(ctx, resp): ResponseSeed,
) -> impl IntoResponse {
  const POPOVER_CLASS: &str =
    "absolute right-0 top-[calc(100%+(var(--spacing)*4))] min-w-56 \
     elevation-lv1 z-50 p-2 flex flex-col gap-1 leading-none";

  let Some(auth_state) = ctx.auth_state() else {
    return resp.into_stream(html! {
      div data-popover class=(POPOVER_CLASS) {
        (form_rejection(UNAUTHENTICATED_MESSAGE))
      }
    });
  };
  let auth_user = auth_state.auth_user();

  let active_org = auth_user.active_org();
  let org_rows = auth_user
    .iter_orgs()
    .map(|o| org_row(ctx.clone(), auth_user.clone(), o, o == active_org))
    .collect::<Vec<_>>();

  resp.into_stream(html! {
    div data-popover class=(POPOVER_CLASS) {
      @ for org_row in org_rows {
        (org_row)
      }
      div class="p-1" {
        div class="h-0 border-t-2 border-base-6/75" {}
      }
      (extra_rows(auth_state.active_org_url_hook()))
    }
  })
}

fn org_row(
  ctx: Ctx,
  auth_user: AuthUser,
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

  let descriptor_suspense = ctx.suspend(
    move |ctx| async move {
      let meta = ctx.state().domain.meta();
      match meta.fetch_org_by_id(org_id).await {
        Ok(Some(org)) => PreEscaped(OrgHook::new(org, auth_user).descriptor()),
        Ok(None) => html! { "[unknown]" },
        Err(_) => html! { "[error]" },
      }
    },
    html! { "[loading]" },
  );

  html! {
    a href=(action_href) class=(class) {
      (icon_element)
      span class="flex-1 text-ellipsis" {
        (descriptor_suspense)
      }
    }
  }
}

fn extra_rows(active: OrgUrlHook) -> Markup {
  let active_org_settings_url = active.settings_url();
  const CREATE_ORG_URL: &str = concatcp!(APP_PREFIX, "/org/create_org");

  html! {
    a href=(active_org_settings_url) class="btn-link btn-link-secondary btn-link-tight" {
      div class="size-5 shrink-0 stroke-base-11 stroke-[2.0]" { (cog_6_tooth_hero_icon()) }
      span class="flex-1 text-ellipsis" {
        "Org Settings"
      }
    }
    a href=(CREATE_ORG_URL) class="btn-link btn-link-secondary btn-link-tight" {
      div class="size-5 shrink-0 stroke-base-11 stroke-[2.0]" { (plus_hero_icon()) }
      span class="flex-1 text-ellipsis" {
        "Create Org"
      }
    }
  }
}

// This handler is called not from HTMX, but is hit with a regular <a> tag. It
// navigates the user by redirecting them with a status redirect and location
// header, after the mutation is complete.
async fn org_selector_action(
  ResponseSeed(ctx, resp): ResponseSeed,
  Path(path_map): Path<HashMap<String, String>>,
) -> impl IntoResponse {
  let Some(auth_state) = ctx.auth_state() else {
    return resp
      .into_stream(form_rejection(UNAUTHENTICATED_MESSAGE))
      .into_response();
  };

  let requested_org_param =
    path_map.get("id").expect("missing action id path param");

  // fail if ID can't be parsed
  let Ok(requested_org) = requested_org_param.parse::<RecordId<_>>() else {
    return resp
      .into_stream(form_rejection("Could not parse requested org ID"))
      .into_response();
  };

  let result = action(ctx, auth_state.auth_user(), requested_org).await;
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
  ctx: Ctx,
  auth_user: AuthUser,
  requested_org: RecordId<Org>,
) -> Result<RecordId<Org>, &'static str> {
  let domain_service = ctx.state().domain.clone();

  domain_service
    .switch_active_org(auth_user.id, requested_org)
    .await
    .map_err(|e| match e {
      UpdateActiveOrgError::InvalidOrg(_) => "Could not switch to this org",
      e => {
        tracing::error!("failed to fetch org: {e}");
        INTERNAL_ERROR_MESSAGE
      }
    })
}
