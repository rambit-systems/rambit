use std::str::FromStr;

use leptos::prelude::*;
use models::{EntityName, Visibility};

use super::{NAME_FIELD_NAME, VISIBILITY_FIELD_NAME};
use crate::form_feedback_text::*;

pub(super) fn extract_fields() -> Result<(EntityName, Visibility), &'static str>
{
  let form_data = leptos_router::hooks::use_query_map().get();

  let Some(name) = form_data.get(NAME_FIELD_NAME) else {
    return Err(const_format::formatcp!(
      "The validation request did not contain the \"{NAME_FIELD_NAME}\" field \
       :/"
    ));
  };
  if name.is_empty() {
    return Err(EMPTY_NAME_MESSAGE);
  }
  let name = EntityName::new(name);

  let Some(visibility) = form_data.get(VISIBILITY_FIELD_NAME) else {
    return Err(const_format::formatcp!(
      "The validation request did not contain the \"{VISIBILITY_FIELD_NAME}\" \
       field :/"
    ));
  };
  let Ok(visibility) = Visibility::from_str(&visibility) else {
    return Err(const_format::formatcp!(
      "Failed to parse the value of the \"{VISIBILITY_FIELD_NAME}\" field :/"
    ));
  };

  Ok((name, visibility))
}
