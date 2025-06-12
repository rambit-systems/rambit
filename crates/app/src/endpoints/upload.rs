use std::{collections::HashMap, io, str::FromStr};

use axum::{
  Json,
  body::Body,
  extract::{Path, State},
  http::{HeaderMap, StatusCode},
  response::IntoResponse,
};
use prime_domain::{
  belt::{self, Belt, StreamExt},
  models::{
    User,
    dvf::{self, EntityName, LaxSlug, RecordId, StrictSlug},
  },
  upload::UploadRequest,
};

use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn upload(
  Path(params): Path<HashMap<String, String>>,
  headers: HeaderMap,
  State(app_state): State<AppState>,
  body: Body,
) -> impl IntoResponse {
  let cache_name = params
    .get("cache_name")
    .expect("upload route param names are malformed")
    .clone();
  if dvf::strict::strict_slugify(&cache_name) != cache_name {
    return (
      StatusCode::BAD_REQUEST,
      format!("Cache name is malformed: `{cache_name}`"),
    )
      .into_response();
  }
  let cache_name = EntityName::new(StrictSlug::new(cache_name));

  let desired_path = params
    .get("path")
    .expect("upload route param names are malformed")
    .clone();
  if dvf::lax::lax_slugify(&desired_path) != desired_path {
    return (
      StatusCode::BAD_REQUEST,
      format!("Path is malformed: `{desired_path}`"),
    )
      .into_response();
  }
  let desired_path = LaxSlug::new(desired_path);

  let target_store = params.get("target_store").cloned();
  if let Some(target_store) = &target_store {
    if dvf::strict::strict_slugify(target_store) != *target_store {
      return (
        StatusCode::BAD_REQUEST,
        format!("Target store is malformed: `{target_store}`"),
      )
        .into_response();
    }
  }
  let target_store =
    target_store.map(|ts| EntityName::new(StrictSlug::new(ts)));

  let user_id = match headers.get("user_id") {
    Some(hv) => match hv.to_str() {
      Ok(s) => match RecordId::<User>::from_str(s) {
        Ok(user_id) => user_id,
        Err(_) => {
          return (StatusCode::BAD_REQUEST, "`user_id` malformed: `{s}`")
            .into_response();
        }
      },
      Err(_) => {
        return (StatusCode::BAD_REQUEST, "`user_id` header is not ASCII")
          .into_response();
      }
    },
    None => {
      return (StatusCode::BAD_REQUEST, "`user_id` header missing")
        .into_response();
    }
  };

  let data = Belt::from_stream(
    body
      .into_data_stream()
      .map(|res| res.map_err(|e| io::Error::other(e.to_string()))),
    Some(belt::DEFAULT_CHUNK_SIZE),
  );

  let upload_req = UploadRequest {
    data,
    auth: user_id,
    cache_name,
    desired_path,
    target_store,
  };

  let upload_resp = app_state.prime_domain.upload(upload_req).await;

  match upload_resp {
    Ok(resp) => Json(resp).into_response(),
    Err(err) => format!("{err:?}").into_response(),
  }
}
