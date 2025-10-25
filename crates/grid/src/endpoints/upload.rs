use std::{collections::HashMap, io};

use axum::{
  Json,
  body::Body,
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
};
use domain::{
  belt::{self, Belt, StreamExt},
  models::NarDeriverData,
  upload::UploadRequest,
};

use super::extractors::{
  CacheListExtractor, DeriverStorePathExtractor, StorePathExtractor,
  TargetStoreExtractor, UserAuthExtractor,
};
use crate::app_state::AppState;

#[allow(clippy::too_many_arguments)]
#[axum::debug_handler]
pub async fn upload(
  Query(query): Query<HashMap<String, String>>,
  CacheListExtractor(caches): CacheListExtractor,
  store_path: StorePathExtractor,
  deriver_store_path: DeriverStorePathExtractor,
  target_store: TargetStoreExtractor,
  UserAuthExtractor(user): UserAuthExtractor,
  State(app_state): State<AppState>,
  body: Body,
) -> impl IntoResponse {
  let Some(deriver_system) = query.get("deriver_system") else {
    return (StatusCode::BAD_REQUEST, "Deriver system is missing")
      .into_response();
  };
  if deriver_system.is_empty() {
    return (StatusCode::BAD_REQUEST, "Deriver system is missing")
      .into_response();
  }

  // WARNING: the system field is totally unvalidated at this point.
  let deriver_data = NarDeriverData {
    system:  Some(deriver_system.clone()),
    deriver: Some(deriver_store_path.value().clone()),
  };

  let nar_contents = Belt::from_stream(
    body
      .into_data_stream()
      .map(|res| res.map_err(|e| io::Error::other(e.to_string()))),
    // Some(belt::DEFAULT_CHUNK_SIZE),
    None,
  );

  let upload_req = UploadRequest {
    auth: user.id,
    target_store: target_store.value().clone(),
    nar_contents,
    caches,
    store_path: store_path.value().clone(),
    deriver_data,
  };

  let upload_plan = match app_state.domain.plan_upload(upload_req).await {
    Ok(plan) => plan,
    Err(err) => {
      return format!("{err:?}").into_response();
    }
  };
  match app_state.domain.execute_upload(upload_plan).await {
    Ok(resp) => Json(resp).into_response(),
    Err(err) => format!("{err:?}").into_response(),
  }
}
