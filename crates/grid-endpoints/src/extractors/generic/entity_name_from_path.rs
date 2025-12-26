use std::{collections::HashMap, marker::PhantomData};

use axum::{
  extract::{FromRequestParts, Path},
  http::{StatusCode, request::Parts},
};
use domain::models::{EntityName, Slug};

use super::PathParameter;

pub struct EntityNameFromPathExtractor<P>(EntityName, PhantomData<P>);

impl<P> EntityNameFromPathExtractor<P> {
  pub fn value(&self) -> &EntityName { &self.0 }
}

impl<S: Send + Sync, P: PathParameter> FromRequestParts<S>
  for EntityNameFromPathExtractor<P>
{
  type Rejection = (StatusCode, String);

  async fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> Result<Self, Self::Rejection> {
    let path =
      <Path<HashMap<String, String>> as FromRequestParts<S>>::from_request_parts(parts, _state)
        .await
        .unwrap();

    let Some(value) = path.get(P::PARAM_NAME) else {
      return Err((
        StatusCode::BAD_REQUEST,
        format!(
          "{desc} is missing (path param `{p_name}`)",
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
    if Slug::new(value).as_ref() != value {
      return Err((
        StatusCode::BAD_REQUEST,
        format!("{desc} is malformed: `{value}`", desc = P::DESCRIPTION),
      ));
    }
    let value = EntityName::new(value);

    Ok(Self(value, PhantomData))
  }
}
