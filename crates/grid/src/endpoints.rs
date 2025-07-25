mod authenticate;
mod download;
mod extractors;
mod narinfo;
mod nix_cache_info;
mod upload;

use axum::{
  Router,
  response::IntoResponse,
  routing::{get, post},
};

use self::{
  authenticate::authenticate, download::download, narinfo::narinfo,
  nix_cache_info::nix_cache_info, upload::upload,
};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn root() -> impl IntoResponse {
  "You've reached the root endpoint of the Rambit API.\nYou probably meant to \
   go somewhere else."
}

pub fn router(app_state: AppState) -> Router {
  axum::Router::new()
    .route("/", get(root))
    .route("/authenticate", get(authenticate).post(authenticate))
    .route("/upload", post(upload))
    .route("/c/{cache_name}/nix-cache-info", get(nix_cache_info))
    .route("/c/{cache_name}/download/{store_path}", get(download))
    .route("/c/{cache_name}/narinfo/{store_path}", get(narinfo))
    .with_state(app_state)
}
