use std::{collections::HashMap, io};

use axum::{
  Json,
  body::Body,
  extract::{Query, State},
  http::StatusCode,
  response::IntoResponse,
};
use prime_domain::{
  belt::{self, Belt, StreamExt},
  models::{
    NarDeriverData,
    dvf::{self, EntityName, StrictSlug},
  },
  upload::UploadRequest,
};

use super::extractors::{
  CacheListExtractor, DeriverStorePathExtractor, StorePathExtractor,
  UserIdExtractor,
};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn upload(
  Query(query): Query<HashMap<String, String>>,
  CacheListExtractor(caches): CacheListExtractor,
  store_path: StorePathExtractor,
  deriver_store_path: DeriverStorePathExtractor,
  UserIdExtractor(user_id): UserIdExtractor,
  State(app_state): State<AppState>,
  body: Body,
) -> impl IntoResponse {
  let Some(target_store) = query.get("target_store") else {
    return (StatusCode::BAD_REQUEST, "Target store is missing")
      .into_response();
  };
  if target_store.is_empty() {
    return (StatusCode::BAD_REQUEST, "Target store is missing")
      .into_response();
  }
  if dvf::strict::strict_slugify(target_store) != *target_store {
    return (
      StatusCode::BAD_REQUEST,
      format!("Target store is malformed: `{target_store}`"),
    )
      .into_response();
  }
  let target_store = EntityName::new(StrictSlug::new(target_store));

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
    deriver: Some(deriver_store_path.0),
  };

  let nar_contents = Belt::from_stream(
    body
      .into_data_stream()
      .map(|res| res.map_err(|e| io::Error::other(e.to_string()))),
    Some(belt::DEFAULT_CHUNK_SIZE),
  );

  let upload_req = UploadRequest {
    auth: user_id,
    target_store,
    nar_contents,
    caches,
    store_path: store_path.0,
    deriver_data,
  };

  let upload_resp = app_state.prime_domain.upload(upload_req).await;

  match upload_resp {
    Ok(resp) => Json(resp).into_response(),
    Err(err) => format!("{err:?}").into_response(),
  }
}
