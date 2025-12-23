use std::{collections::HashMap, str::FromStr};

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
};
use domain::{models::Digest, narinfo::NarinfoRequest};
use grid_state::AppState;

use super::extractors::{CacheNameExtractor, UserAuthExtractor};

#[axum::debug_handler]
pub async fn narinfo(
  cache_name: CacheNameExtractor,
  Path(params): Path<HashMap<String, String>>,
  user: Option<UserAuthExtractor>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let digest = match params
    .get("digest_with_suffix")
    .expect("upload route param names are malformed")
    .strip_suffix(".narinfo")
  {
    Some(d) => d,
    None => {
      return (
        StatusCode::NOT_FOUND,
        "Expected a digest ending in \".narinfo\"",
      )
        .into_response();
    }
  };
  let digest = match Digest::from_str(digest) {
    Ok(digest) => digest,
    Err(_) => {
      return (
        StatusCode::BAD_REQUEST,
        format!("Digest is malformed: `{digest}`"),
      )
        .into_response();
    }
  };

  let narinfo_req = NarinfoRequest {
    auth: user.map(|e| e.0.id),
    cache_name: cache_name.value().clone(),
    digest,
  };

  let narinfo_resp = app_state.domain.narinfo(narinfo_req).await;

  match narinfo_resp {
    Ok(resp) => resp.narinfo().to_string().into_response(),
    Err(err) => format!("{err:#?}").into_response(),
  }
}
