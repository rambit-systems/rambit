use axum::{
  self,
  body::Body,
  extract::{Request, State},
  handler::Handler,
  response::IntoResponse,
};
use grid_state::AppState;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub(crate) async fn static_asset_handler(
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> impl IntoResponse {
  let service = ServeDir::new(app_state.serve_config.static_asset_dir.clone())
    .not_found_service(app::fallback_handler.with_state(app_state));
  let Ok(response) = service.oneshot(request).await;
  response
}
