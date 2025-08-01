use std::{collections::HashMap, marker::PhantomData};

use axum::{
  extract::{FromRequestParts, Query},
  http::{StatusCode, request::Parts},
};
use prime_domain::models::StorePath;

use super::QueryParameter;

pub struct StorePathFromQueryExtractor<P>(StorePath<String>, PhantomData<P>);

impl<P> StorePathFromQueryExtractor<P> {
  pub fn value(&self) -> &StorePath<String> { &self.0 }
}

impl<S: Sync, P: QueryParameter> FromRequestParts<S>
  for StorePathFromQueryExtractor<P>
{
  type Rejection = (StatusCode, String);

  async fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> Result<Self, Self::Rejection> {
    let query =
      Query::<HashMap<String, String>>::try_from_uri(&parts.uri).unwrap();

    let Some(value) = query.get(P::PARAM_NAME) else {
      return Err((
        StatusCode::BAD_REQUEST,
        format!(
          "{desc} is missing (query param `{p_name}`)",
          desc = P::DESCRIPTION,
          p_name = P::PARAM_NAME
        ),
      ));
    };
    if value.is_empty() {
      return Err((
        StatusCode::BAD_REQUEST,
        format!("{desc} is empty", desc = P::DESCRIPTION),
      ));
    }
    let value = StorePath::from_bytes(value.as_bytes()).map_err(|_| {
      (
        StatusCode::BAD_REQUEST,
        format!("{desc} is malformed: `{value}`", desc = P::DESCRIPTION),
      )
    })?;

    Ok(Self(value, PhantomData))
  }
}
