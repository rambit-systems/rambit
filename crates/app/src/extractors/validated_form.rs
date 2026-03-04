use std::collections::HashMap;

use axum::{
  extract::{FromRequest, Request},
  response::{Html, IntoResponse, Response},
};
use models::{
  EmailAddress, EmailAddressError, EntityName, HumanName, HumanNameError,
  UserSubmittedAuthCredentials, Visibility,
};

use crate::{components::form_result::form_rejection, form_feedback_text::*};

const NAME_FIELD_NAME: &str = "name";
const EMAIL_FIELD_NAME: &str = "email";
const VISIBILITY_FIELD_NAME: &str = "visibility";
const PASSWORD_FIELD_NAME: &str = "password";
const CONFIRM_FIELD_NAME: &str = "confirm";

/// Implement this for any type that can be extracted from a form field.
pub trait FromFormField: Sized {
  /// The form field name (e.g., "email")
  const FIELD_NAME: &'static str;

  /// Try to parse from a raw string value. Return a user-facing error fragment
  /// on failure.
  fn from_field(value: &str) -> Result<Self, String>;
}

impl FromFormField for HumanName {
  const FIELD_NAME: &'static str = NAME_FIELD_NAME;

  fn from_field(value: &str) -> Result<Self, String> {
    if value.is_empty() {
      return Err(EMPTY_NAME_MESSAGE.to_owned());
    }
    HumanName::try_new(value).map_err(|e| match e {
      HumanNameError::Empty => unreachable!(),
      HumanNameError::TooLong => NAME_TOO_LONG_MESSAGE.to_owned(),
    })
  }
}

impl FromFormField for EntityName {
  const FIELD_NAME: &'static str = NAME_FIELD_NAME;

  fn from_field(value: &str) -> Result<Self, String> {
    if value.is_empty() {
      return Err(EMPTY_ENTITY_NAME_MESSAGE.to_owned());
    }
    Ok(EntityName::new(value))
  }
}

impl FromFormField for EmailAddress {
  const FIELD_NAME: &'static str = EMAIL_FIELD_NAME;

  fn from_field(value: &str) -> Result<Self, String> {
    if value.is_empty() {
      return Err(EMPTY_EMAIL_MESSAGE.to_owned());
    }
    EmailAddress::try_new(value).map_err(|e| match e {
      EmailAddressError::InvalidEmail => MALFORMED_EMAIL_MESSAGE.to_owned(),
      EmailAddressError::TooLong => EMAIL_TOO_LONG_MESSAGE.to_owned(),
    })
  }
}

impl FromFormField for Visibility {
  const FIELD_NAME: &'static str = VISIBILITY_FIELD_NAME;

  fn from_field(value: &str) -> Result<Self, String> {
    value
      .parse()
      .map_err(|()| format!("Invalid visibility value: \"{value}\" :/"))
  }
}

/// For types that need access to the full form map (multi-field validation).
pub trait FromFormMap: Sized {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String>;
}

impl FromFormMap for UserSubmittedAuthCredentials {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String> {
    let password = map
      .get(PASSWORD_FIELD_NAME)
      .filter(|s| !s.is_empty())
      .ok_or(EMPTY_PASSWORD_MESSAGE)?;
    let Some(confirm) = map.get(CONFIRM_FIELD_NAME) else {
      return Ok(UserSubmittedAuthCredentials::Password {
        password: password.to_owned(),
      });
    };
    if password != confirm {
      return Err(PASSWORD_CONFIRM_MISMATCH_MESSAGE.to_owned());
    }
    Ok(UserSubmittedAuthCredentials::Password {
      password: password.to_owned(),
    })
  }
}

// Blanket impl so single-field types also work via the map interface
impl<T: FromFormField> FromFormMap for T {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String> {
    let value = map
      .get(Self::FIELD_NAME)
      .ok_or_else(|| format!("Missing \"{}\" field :/", T::FIELD_NAME))?;
    Self::from_field(value)
  }
}

/// Marker trait for structs whose fields all implement FromFormMap.
pub trait FromValidatedForm: Sized {
  fn from_form_map(map: &HashMap<String, String>) -> Result<Self, String>;
}

pub struct ValidatedForm<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedForm<T>
where
  S: Send + Sync,
  T: FromValidatedForm,
{
  type Rejection = FormRejection;

  async fn from_request(
    req: Request,
    state: &S,
  ) -> Result<Self, Self::Rejection> {
    // Extract the raw form data first
    let axum::Form(map): axum::Form<HashMap<String, String>> =
      axum::Form::from_request(req, state)
        .await
        .map_err(|_| FormRejection("Invalid form submission".to_owned()))?;

    T::from_form_map(&map)
      .map(ValidatedForm)
      .map_err(FormRejection)
  }
}

/// Returns an HTMX-friendly HTML error fragment.
pub struct FormRejection(String);

impl IntoResponse for FormRejection {
  fn into_response(self) -> Response {
    // Return a 422 with an HTML fragment your HTMX swap target can display
    Html(form_rejection(self.0).0).into_response()
  }
}
