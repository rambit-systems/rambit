use leptos::prelude::*;
use models::{EmailAddress, UserSubmittedAuthCredentials};

use super::{EMAIL_FIELD_NAME, PASSWORD_FIELD_NAME};
use crate::form_feedback_text::*;

pub(super) fn extract_fields(
) -> Result<(EmailAddress, UserSubmittedAuthCredentials), &'static str> {
  let form_data = leptos_router::hooks::use_query_map().get();

  let Some(email) = form_data.get(EMAIL_FIELD_NAME) else {
    return Err(const_format::formatcp!(
      "The login request did not contain the \"{EMAIL_FIELD_NAME}\" field :/"
    ));
  };
  if email.is_empty() {
    return Err(EMPTY_EMAIL_MESSAGE);
  }
  let email = match EmailAddress::try_new(email) {
    Ok(email) => email,
    Err(models::EmailAddressError::InvalidEmail) => {
      return Err(MALFORMED_EMAIL_MESSAGE);
    }
    Err(models::EmailAddressError::TooLong) => {
      return Err(EMAIL_TOO_LONG_MESSAGE);
    }
  };

  let Some(password) = form_data.get(PASSWORD_FIELD_NAME) else {
    return Err(const_format::formatcp!(
      "The login request did not contain the \"{PASSWORD_FIELD_NAME}\" field \
       :/"
    ));
  };
  if password.is_empty() {
    return Err(EMPTY_PASSWORD_MESSAGE);
  }
  let creds = UserSubmittedAuthCredentials::Password {
    password: password.to_owned(),
  };

  Ok((email, creds))
}
