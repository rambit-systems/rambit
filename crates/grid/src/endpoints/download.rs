use std::collections::HashMap;

use axum::{
  body::Body,
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
};
use prime_domain::{download::DownloadRequest, models::StorePath};

use super::extractors::{CacheNameExtractor, UserIdExtractor};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn download(
  cache_name: CacheNameExtractor,
  user_id: Option<UserIdExtractor>,
  Path(params): Path<HashMap<String, String>>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let store_path = params
    .get("store_path")
    .expect("upload route param names are malformed")
    .clone();
  let store_path = match StorePath::from_bytes(store_path.as_bytes()) {
    Ok(store_path) => store_path,
    Err(_) => {
      return (
        StatusCode::BAD_REQUEST,
        format!("Store path is malformed: `{store_path}`"),
      )
        .into_response();
    }
  };

  let download_req = DownloadRequest {
    auth: user_id.map(|e| e.0),
    cache_name: cache_name.value().clone(),
    store_path,
  };

  let download_resp = app_state.prime_domain.download(download_req).await;

  match download_resp {
    Ok(resp) => Body::from_stream(resp.data).into_response(),
    Err(err) => format!("{err:#?}").into_response(),
  }
}
