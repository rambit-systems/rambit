use std::{collections::HashMap, marker::PhantomData};

use axum::{
  extract::{FromRequestParts, Query},
  http::{StatusCode, request::Parts},
  response::{IntoResponse, Response},
};
use prime_domain::models::dvf::{self, EntityName, StrictSlug};

use super::QueryParameter;

pub struct EntityNameFromQueryExtractor<P>(EntityName, PhantomData<P>);

impl<P> EntityNameFromQueryExtractor<P> {
  pub fn value(&self) -> &EntityName { &self.0 }
}

impl<S: Sync, P: QueryParameter> FromRequestParts<S>
  for EntityNameFromQueryExtractor<P>
{
  type Rejection = EntityNameFromQueryRejection;

  async fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> Result<Self, Self::Rejection> {
    let query =
      Query::<HashMap<String, String>>::try_from_uri(&parts.uri).unwrap();

    let Some(value) = query.get(P::PARAM_NAME) else {
      return Err(EntityNameFromQueryRejection(
        StatusCode::BAD_REQUEST,
        format!(
          "{desc} is missing (query param `{p_name}`)",
          desc = P::DESCRIPTION,
          p_name = P::PARAM_NAME
        ),
      ));
    };
    if value.is_empty() {
      return Err(EntityNameFromQueryRejection(
        StatusCode::BAD_REQUEST,
        format!("{desc} is empty", desc = P::DESCRIPTION),
      ));
    }
    if dvf::strict::strict_slugify(value) != *value {
      return Err(EntityNameFromQueryRejection(
        StatusCode::BAD_REQUEST,
        format!("{desc} is malformed: `{value}`", desc = P::DESCRIPTION),
      ));
    }
    let value = EntityName::new(StrictSlug::new(value));

    Ok(Self(value, PhantomData))
  }
}

pub struct EntityNameFromQueryRejection(StatusCode, String);

impl IntoResponse for EntityNameFromQueryRejection {
  fn into_response(self) -> Response { (self.0, self.1).into_response() }
}
