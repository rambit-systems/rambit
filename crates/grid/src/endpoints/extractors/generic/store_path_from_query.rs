use std::{collections::HashMap, marker::PhantomData};

use axum::{
  extract::{FromRequestParts, Query},
  http::{StatusCode, request::Parts},
  response::{IntoResponse, Response},
};
use prime_domain::models::StorePath;

pub trait QueryParameter {
  const PARAM_NAME: &'static str;
  const DESCRIPTION: &'static str;
}

pub struct StorePathFromQueryExtractor<P: QueryParameter>(
  pub StorePath<String>,
  PhantomData<P>,
);

impl<S: Sync, P: QueryParameter> FromRequestParts<S>
  for StorePathFromQueryExtractor<P>
{
  type Rejection = StorePathFromQueryRejection;

  async fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> Result<Self, Self::Rejection> {
    let query =
      Query::<HashMap<String, String>>::try_from_uri(&parts.uri).unwrap();

    let Some(value) = query.get(P::PARAM_NAME) else {
      return Err(StorePathFromQueryRejection(
        StatusCode::BAD_REQUEST,
        format!(
          "{desc} is missing (query param `{p_name}`)",
          desc = P::DESCRIPTION,
          p_name = P::PARAM_NAME
        ),
      ));
    };
    if value.is_empty() {
      return Err(StorePathFromQueryRejection(
        StatusCode::BAD_REQUEST,
        format!("{desc} is empty", desc = P::DESCRIPTION),
      ));
    }
    let value = StorePath::from_bytes(value.as_bytes()).map_err(|_| {
      StorePathFromQueryRejection(
        StatusCode::BAD_REQUEST,
        format!("{desc} is malformed: `{value}`", desc = P::DESCRIPTION),
      )
    })?;

    Ok(Self(value, PhantomData))
  }
}

pub struct StorePathFromQueryRejection(StatusCode, String);

impl IntoResponse for StorePathFromQueryRejection {
  fn into_response(self) -> Response { (self.0, self.1).into_response() }
}
