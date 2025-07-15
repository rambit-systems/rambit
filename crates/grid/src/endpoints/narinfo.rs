use std::collections::HashMap;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
};
use prime_domain::{
  models::{
    StorePath,
    dvf::{self, EntityName, StrictSlug},
  },
  narinfo::NarinfoRequest,
};

use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn narinfo(
  Path(params): Path<HashMap<String, String>>,
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

  let narinfo_req = NarinfoRequest {
    cache_name,
    store_path,
  };

  let narinfo_resp = app_state.prime_domain.narinfo(narinfo_req).await;

  match narinfo_resp {
    Ok(resp) => resp.narinfo().to_string().into_response(),
    Err(err) => format!("{err:#?}").into_response(),
  }
}
