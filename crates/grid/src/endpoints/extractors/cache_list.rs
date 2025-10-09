use std::collections::HashMap;

use axum::{
  extract::{FromRequestParts, Query},
  http::{StatusCode, request::Parts},
};
use domain::models::dvf::{self, EntityName, StrictSlug};

const CACHE_LIST_QUERY_PARAM: &str = "caches";

pub struct CacheListExtractor(pub Vec<EntityName>);

impl<S: Sync> FromRequestParts<S> for CacheListExtractor {
  type Rejection = (StatusCode, String);

  async fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> Result<Self, Self::Rejection> {
    let query =
      Query::<HashMap<String, String>>::try_from_uri(&parts.uri).unwrap();

    let Some(value) = query.get("caches") else {
      return Err((
        StatusCode::BAD_REQUEST,
        format!(
          "Cache list is missing (query param `{CACHE_LIST_QUERY_PARAM}`)"
        ),
      ));
    };

    if value.is_empty() {
      return Err((
        StatusCode::BAD_REQUEST,
        format!("Cache list is empty (query param `{CACHE_LIST_QUERY_PARAM}`)"),
      ));
    }

    let caches = value
      .split(",")
      .map(|c| {
        if c.is_empty() {
          return Err((
            StatusCode::BAD_REQUEST,
            "Empty cache name found".to_owned(),
          ));
        }
        match dvf::strict::strict_slugify(c) == c {
          true => Ok(EntityName::new(StrictSlug::new(c))),
          false => Err((
            StatusCode::BAD_REQUEST,
            format!("Cache name is malformed: `{c}`"),
          )),
        }
      })
      .try_collect::<Vec<_>>()?;
    Ok(Self(caches))
  }
}
