mod upload;

use axum::{
  Router,
  response::IntoResponse,
  routing::{get, post},
};

use self::upload::upload;
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn root() -> impl IntoResponse {
  "You've reached the root endpoint of the Rambit API.\nYou probably meant to \
   go somewhere else."
}

pub fn router(app_state: AppState) -> Router {
  axum::Router::new()
    .route("/", get(root))
    .route("/upload/{cache_name}/{path}", post(upload))
    .route("/upload/{cache_name}/{path}/{target_store}", post(upload))
    .with_state(app_state)
}
