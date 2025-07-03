use std::{collections::HashMap, str::FromStr};

use axum::{
  body::Body,
  extract::{Path, State},
  http::{HeaderMap, StatusCode},
  response::IntoResponse,
};
use kv::LaxSlug;
use prime_domain::{
  download::DownloadRequest,
  models::{
    User,
    dvf::{self, EntityName, RecordId, StrictSlug},
  },
};

use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn download(
  Path(params): Path<HashMap<String, String>>,
  headers: HeaderMap,
  State(app_state): State<AppState>,
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

  let user_id = match headers.get("user_id") {
    Some(hv) => match hv.to_str() {
      Ok(s) => match RecordId::<User>::from_str(s) {
        Ok(user_id) => Some(user_id),
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
    None => None,
  };

  let download_req = DownloadRequest {
    auth: user_id,
    cache_name,
    desired_path,
  };

  let download_resp = app_state.prime_domain.download(download_req).await;

  match download_resp {
    Ok(resp) => Body::from_stream(resp.data).into_response(),
    Err(err) => format!("{err:?}").into_response(),
  }
}
