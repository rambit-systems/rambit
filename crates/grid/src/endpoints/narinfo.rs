use std::collections::HashMap;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
};
use prime_domain::{models::StorePath, narinfo::NarinfoRequest};

use super::extractors::{CacheNameExtractor, UserIdExtractor};
use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn narinfo(
  cache_name: CacheNameExtractor,
  Path(params): Path<HashMap<String, String>>,
  user_id: Option<UserIdExtractor>,
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

  let user_id = user_id.map(|e| e.0);

  let narinfo_req = NarinfoRequest {
    auth: user_id,
    cache_name: cache_name.value().clone(),
    store_path,
  };

  let narinfo_resp = app_state.prime_domain.narinfo(narinfo_req).await;

  match narinfo_resp {
    Ok(resp) => resp.narinfo().to_string().into_response(),
    Err(err) => format!("{err:#?}").into_response(),
  }
}
