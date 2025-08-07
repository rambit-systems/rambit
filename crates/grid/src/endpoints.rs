mod authenticate;
mod download;
mod extractors;
mod narinfo;
mod nix_cache_info;
mod signup;
mod upload;

use axum::{
  Router,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
};

use self::{
  authenticate::{authenticate, deauthenticate},
  download::download,
  narinfo::narinfo,
  nix_cache_info::nix_cache_info,
  signup::signup,
  upload::upload,
};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn root() -> impl IntoResponse {
  "You've reached the root endpoint of the Rambit API.\nYou probably meant to \
   go somewhere else."
}

#[axum::debug_handler]
pub async fn fallback() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "endpoint not found")
}

pub fn router() -> Router<AppState> {
  axum::Router::new()
    .route("/", get(root))
    .route("/signup", post(signup))
    .route("/authenticate", post(authenticate))
    .route("/deauthenticate", post(deauthenticate))
    .route("/upload", post(upload))
    .route("/c/{cache_name}/nix-cache-info", get(nix_cache_info))
    .route("/c/{cache_name}/download/{store_path}", get(download))
    .route("/c/{cache_name}/{digest_with_suffix}", get(narinfo))
    .fallback(fallback)
}
