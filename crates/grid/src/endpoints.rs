mod download;
mod extractors;
mod narinfo;
mod upload;

use axum::{
  Router,
  response::IntoResponse,
  routing::{get, post},
};

use self::{download::download, narinfo::narinfo, upload::upload};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn root() -> impl IntoResponse {
  "You've reached the root endpoint of the Rambit API.\nYou probably meant to \
   go somewhere else."
}

pub fn router(app_state: AppState) -> Router {
  axum::Router::new()
    .route("/", get(root))
    .route("/upload", post(upload))
    .route("/download/{cache_name}/{store_path}", get(download))
    .route("/narinfo/{cache_name}/{store_path}", get(narinfo))
    .with_state(app_state)
}
