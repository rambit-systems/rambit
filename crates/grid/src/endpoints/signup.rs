use auth_domain::{AuthDomainService, AuthSession};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use domain::models::{EmailAddress, HumanName, UserSubmittedAuthCredentials};
use serde::Deserialize;

use crate::util_traits::InternalError;

#[derive(Deserialize)]
pub struct SignupParams {
  name:     Option<String>,
  email:    Option<String>,
  password: Option<String>,
}

#[axum::debug_handler]
pub async fn signup(
  mut auth_session: AuthSession,
  State(auth_domain_service): State<AuthDomainService>,
  Json(params): Json<SignupParams>,
) -> impl IntoResponse {
  let Some(name) = params.name else {
    return (StatusCode::BAD_REQUEST, "Missing `name` field").into_response();
  };
  if name.is_empty() {
    return (StatusCode::BAD_REQUEST, "Empty `name` field").into_response();
  }
  let name = match HumanName::try_new(name) {
    Ok(name) => name,
    Err(_) => {
      return (StatusCode::BAD_REQUEST, "Malformed `name` field")
        .into_response();
    }
  };

  let Some(email) = params.email else {
    return (StatusCode::BAD_REQUEST, "Missing `email` field").into_response();
  };
  if email.is_empty() {
    return (StatusCode::BAD_REQUEST, "Empty `email` field").into_response();
  }
  let email = match EmailAddress::try_new(email) {
    Ok(email) => email,
    Err(_) => {
      return (StatusCode::BAD_REQUEST, "Malformed `email` field")
        .into_response();
    }
  };

  let Some(password) = params.password else {
    return (StatusCode::BAD_REQUEST, "Missing `password` field")
      .into_response();
  };
  if password.is_empty() {
    return (StatusCode::BAD_REQUEST, "Empty `password` field").into_response();
  }

  let creds = UserSubmittedAuthCredentials::Password { password };

  if let Err(e) = auth_domain_service
    .user_signup(name, email.clone(), creds.clone())
    .await
  {
    return e.internal("failed to sign up");
  };

  let user = match auth_session.authenticate((email, creds)).await {
    Ok(Some(user)) => user,
    Ok(None) => {
      return (StatusCode::UNAUTHORIZED, Json(())).into_response();
    }
    Err(e) => {
      return e.internal("failed to authenticate");
    }
  };

  match auth_session.login(&user).await {
    Ok(_) => (StatusCode::OK, Json(user.id)).into_response(),
    Err(e) => e.internal("failed to login"),
  }
}
