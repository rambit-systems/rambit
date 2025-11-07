use std::collections::HashMap;

use axum::{
  body::Body,
  extract::{Path, State},
  http::{StatusCode, header::CONTENT_LENGTH},
  response::IntoResponse,
};
use domain::{
  download::{DownloadRequest, DownloadResponse},
  models::StorePath,
};

use super::extractors::{CacheNameExtractor, UserAuthExtractor};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn download(
  cache_name: CacheNameExtractor,
  user: Option<UserAuthExtractor>,
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
    auth: user.map(|e| e.0.id),
    cache_name: cache_name.value().clone(),
    store_path,
  };

  let download_plan = match app_state.domain.plan_download(download_req).await {
    Ok(plan) => plan,
    Err(err) => {
      return format!("{err:#?}").into_response();
    }
  };

  let download_resp = app_state.domain.execute_download(download_plan).await;

  match download_resp {
    Ok(DownloadResponse { data, file_size }) => (
      [(CONTENT_LENGTH, file_size.inner().to_string())],
      Body::from_stream(data),
    )
      .into_response(),
    Err(err) => format!("{err:#?}").into_response(),
  }
}
