use std::{collections::HashMap, str::FromStr};

use axum::{
  extract::{FromRequestParts, OptionalFromRequestParts, Path},
  http::request::Parts,
};
use models::{Org, RecordId};
use tracing::debug;

pub struct PathRequestedOrgId(pub RecordId<Org>);

impl<S: Send + Sync> OptionalFromRequestParts<S> for PathRequestedOrgId {
  type Rejection = <Path<String> as FromRequestParts<S>>::Rejection;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &S,
  ) -> Result<Option<Self>, Self::Rejection> {
    let Path(params) = <Path<HashMap<String, String>> as FromRequestParts<
    S,
  >>::from_request_parts(parts, state)
  .await
  .expect("unreachable: hashmap can always deserialize path");
    let Some(segment) = params.get("org") else {
      return Ok(None);
    };

    match RecordId::from_str(segment) {
      Ok(r) => Ok(Some(Self(r))),
      Err(_) => {
        debug!("failed to parse requested org string {segment:?} as record ID");
        Ok(None)
      }
    }
  }
}
