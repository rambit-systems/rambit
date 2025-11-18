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
use drop_stream::StreamDropCallbackExt;

use super::extractors::{CacheNameExtractor, UserAuthExtractor};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn download(
  cache_name: CacheNameExtractor,
  user: Option<UserAuthExtractor>,
  Path(params): Path<HashMap<String, String>>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  // get store path from path param
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

  // build download request
  let download_req = DownloadRequest {
    auth: user.map(|e| e.0.id),
    cache_name: cache_name.value().clone(),
    store_path,
  };

  // plan download operation
  let download_plan = match app_state.domain.plan_download(download_req).await {
    Ok(plan) => plan,
    Err(err) => {
      return format!("{err:#?}").into_response();
    }
  };

  // destructure download response, short-circuiting error
  let DownloadResponse {
    data,
    file_size,
    egress_event,
  } = match app_state.domain.execute_download(download_plan).await {
    Ok(resp) => resp,
    Err(err) => {
      return format!("{err:#?}").into_response();
    }
  };

  // prepare a future to run when the stream is dropped, which sends the
  // egress event with the consumed byte counter
  let egress_counter = data.counter();
  let metrics_domain = app_state.metrics_domain.clone();
  let stream_drop_future = async move {
    metrics_domain
      .send_event(egress_event.stamp_with_now(egress_counter.get()))
      .await;
  };

  (
    [(CONTENT_LENGTH, file_size.inner().to_string())],
    Body::from_stream(data.on_drop_async(stream_drop_future)),
  )
    .into_response()
}
