use auth_domain::AuthSession;
use axum::{
  body::Body,
  extract::{Request, State},
  response::IntoResponse,
};
use leptos::prelude::provide_context;

use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn leptos_routes_handler(
  auth_session: AuthSession,
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> axum::response::Response {
  let leptos_options = app_state.leptos_options.clone();
  leptos_axum::render_app_to_stream_with_context(
    context_provider(app_state.clone(), auth_session),
    move || site_app::shell(leptos_options.clone()),
  )(request)
  .await
  .into_response()
}

fn context_provider(
  app_state: AppState,
  auth_session: AuthSession,
) -> impl Fn() + Clone {
  move || {
    provide_context(app_state.prime_domain.clone());
    provide_context(app_state.auth_domain.clone());
    provide_context(prime_domain::models::AuthStatus(
      auth_session.user.clone(),
    ));
  }
}
