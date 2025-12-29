use auth_domain::AuthSession;
use axum::{
  self, RequestExt,
  body::Body,
  extract::{Request, State},
  handler::Handler,
  http::Method,
  response::IntoResponse,
};
use grid_state::AppState;
use http::Uri;
use leptos::prelude::provide_context;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::debug;

fn leptos_method_from_axum_method(
  input: axum::http::Method,
) -> leptos_router::Method {
  match input {
    axum::http::Method::GET => leptos_router::Method::Get,
    axum::http::Method::POST => leptos_router::Method::Post,
    axum::http::Method::PUT => leptos_router::Method::Put,
    axum::http::Method::PATCH => leptos_router::Method::Patch,
    axum::http::Method::DELETE => leptos_router::Method::Delete,
    axum::http::Method::HEAD => {
      // this is because leptos doesn't have a `HEAD` variant, and `HEAD`
      // automatically gets routed to the `GET` handler by axum
      debug!("got HEAD request; providing GET method to leptos context");
      leptos_router::Method::Get
    }
    verb => panic!("non-standard HTTP verb used in request: {verb}"),
  }
}

async fn context_provider_from_request(
  app_state: State<AppState>,
  mut request: Request<Body>,
) -> (Request<Body>, impl Fn() + Clone + Sync + Send + 'static) {
  let Ok(method) = request.extract_parts::<axum::http::Method>().await;
  let auth_session = request
    .extract_parts::<AuthSession>()
    .await
    .expect("failed to extract AuthSession from request");
  let State(app_state) = app_state;

  (request, move || {
    // http method
    provide_context(leptos_method_from_axum_method(method.clone()));

    // all app state
    provide_context(app_state.clone());

    // paddle items
    provide_context(app_state.domain.paddle_client_secret());
    provide_context(app_state.domain.paddle_environment());

    // auth session and user
    provide_context(auth_session.clone());
    if let Some(auth_user) = auth_session.user.clone() {
      provide_context(auth_user);
    }
  })
}

#[axum::debug_handler]
pub(crate) async fn leptos_routes_handler(
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> impl IntoResponse {
  let (request, context_provider) =
    context_provider_from_request(State(app_state), request).await;
  let handler = leptos_axum::render_app_to_stream_with_context(
    context_provider,
    site_app::shell,
  );

  handler(request).await.into_response()
}

#[axum::debug_handler]
pub(crate) async fn leptos_fallback_handler(
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> impl IntoResponse {
  let service = ServeDir::new(app_state.serve_config.static_asset_dir.clone())
    .not_found_service(leptos_routes_handler.with_state(app_state));
  let Ok(response) = service.oneshot(request).await;
  response
}
