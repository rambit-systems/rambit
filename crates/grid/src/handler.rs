// use auth_domain::AuthSession;
use axum::{
  self,
  body::Body,
  extract::{Request, State},
  handler::Handler,
  response::IntoResponse,
};
// use axum_htmx::HxRequest;
use grid_state::AppState;
// use leptos::{either::Either, prelude::provide_context};
use tower::ServiceExt;
use tower_http::services::ServeDir;
// use tracing::debug;

// fn leptos_method_from_axum_method(
//   input: axum::http::Method,
// ) -> leptos_router::Method {
//   match input {
//     axum::http::Method::GET => leptos_router::Method::Get,
//     axum::http::Method::POST => leptos_router::Method::Post,
//     axum::http::Method::PUT => leptos_router::Method::Put,
//     axum::http::Method::PATCH => leptos_router::Method::Patch,
//     axum::http::Method::DELETE => leptos_router::Method::Delete,
//     axum::http::Method::HEAD => {
//       // this is because leptos doesn't have a `HEAD` variant, and `HEAD`
//       // automatically gets routed to the `GET` handler by axum
//       debug!("got HEAD request; providing GET method to leptos context");
//       leptos_router::Method::Get
//     }
//     verb => panic!("non-standard HTTP verb used in request: {verb}"),
//   }
// }

// pub(crate) async fn leptos_routes_handler(
//   State(app_state): State<AppState>,
//   HxRequest(hx_request): HxRequest,
//   auth_session: AuthSession,
//   method: axum::http::Method,
//   request: Request<Body>,
// ) -> impl IntoResponse {
//   let context_provider = move || {
//     // http method
//     provide_context(leptos_method_from_axum_method(method.clone()));

//     // all app state
//     provide_context(app_state.clone());
//     provide_context(app_state.domain.clone());

//     // paddle items
//     provide_context(app_state.domain.paddle_client_secret());
//     provide_context(app_state.domain.paddle_environment());

//     // auth session and user
//     provide_context(auth_session.clone());
//     if let Some(auth_user) = auth_session.user.clone() {
//       provide_context(auth_user);
//     }
//   };

//   match hx_request {
//     true => {
//       // render just the app without the shell, and without suspense
//       let handler = leptos_axum::render_app_async_with_context(
//         context_provider,
//         site_app::App,
//       );
//       handler(request).await.into_response()
//     }
//     false => {
//       let handler = leptos_axum::render_app_to_stream_with_context(
//         context_provider,
//         site_app::shell,
//       );
//       handler(request).await.into_response()
//     }
//   }
// }

pub(crate) async fn static_asset_handler(
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> impl IntoResponse {
  let service = ServeDir::new(app_state.serve_config.static_asset_dir.clone())
    .not_found_service(app::fallback_handler.with_state(app_state));
  let Ok(response) = service.oneshot(request).await;
  response
}
