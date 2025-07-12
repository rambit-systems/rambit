use std::{io, str::FromStr};

use axum::{
  Json,
  body::Body,
  extract::{Query, State},
  http::{HeaderMap, StatusCode},
  response::IntoResponse,
};
use prime_domain::{
  belt::{self, Belt, StreamExt},
  models::{
    NarDeriverData, StorePath, User,
    dvf::{self, EntityName, RecordId, StrictSlug},
  },
  upload::UploadRequest,
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPayload {
  caches:             Vec<String>,
  store_path:         String,
  target_store:       String,
  deriver_system:     String,
  deriver_store_path: String,
}

#[axum::debug_handler]
pub async fn upload(
  Query(params): Query<UploadPayload>,
  headers: HeaderMap,
  State(app_state): State<AppState>,
  body: Body,
) -> impl IntoResponse {
  let caches = params
    .caches
    .into_iter()
    .map(|c| match dvf::strict::strict_slugify(&c) == c {
      true => Ok(EntityName::new(StrictSlug::new(c))),
      false => Err(
        (
          StatusCode::BAD_REQUEST,
          format!("Cache name is malformed: `{c}`"),
        )
          .into_response(),
      ),
    })
    .try_collect::<Vec<_>>();
  let caches = match caches {
    Ok(caches) => caches,
    Err(r) => return r,
  };

  let store_path = match StorePath::from_bytes(params.store_path.as_bytes()) {
    Ok(store_path) => store_path,
    Err(_) => {
      return (
        StatusCode::BAD_REQUEST,
        format!("Store path is malformed: `{}`", params.store_path),
      )
        .into_response();
    }
  };

  if dvf::strict::strict_slugify(&params.target_store) != params.target_store {
    return (
      StatusCode::BAD_REQUEST,
      format!("Target store is malformed: `{}`", params.target_store),
    )
      .into_response();
  }
  let target_store = EntityName::new(StrictSlug::new(params.target_store));

  let Some(user_id_header_data) = headers.get("user_id") else {
    return (StatusCode::BAD_REQUEST, "`user_id` header missing")
      .into_response();
  };
  let Ok(user_id_header_string) = user_id_header_data.to_str() else {
    return (StatusCode::BAD_REQUEST, "`user_id` header is not ASCII")
      .into_response();
  };
  let Ok(user_id) = RecordId::<User>::from_str(user_id_header_string) else {
    return (StatusCode::BAD_REQUEST, "`user_id` malformed: `{s}`")
      .into_response();
  };

  let deriver_store_path =
    match StorePath::from_bytes(params.deriver_store_path.as_bytes()) {
      Ok(deriver_store_path) => deriver_store_path,
      Err(_) => {
        return (
          StatusCode::BAD_REQUEST,
          format!(
            "Deriver store path is malformed: `{}`",
            params.deriver_store_path
          ),
        )
          .into_response();
      }
    };

  // WARNING: the system field is totally unvalidated at this point.
  let deriver_data = NarDeriverData {
    system:  Some(params.deriver_system),
    deriver: Some(deriver_store_path),
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
    store_path,
    deriver_data,
  };

  let upload_resp = app_state.prime_domain.upload(upload_req).await;

  match upload_resp {
    Ok(resp) => Json(resp).into_response(),
    Err(err) => format!("{err:?}").into_response(),
  }
}
