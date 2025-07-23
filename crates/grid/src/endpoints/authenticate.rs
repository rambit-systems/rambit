use auth_domain::AuthSession;
use axum::{Json, http::StatusCode, response::IntoResponse};
use prime_domain::models::{UserSubmittedAuthCredentials, dvf::EmailAddress};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthenticateQueryParams {
  email:    Option<String>,
  password: Option<String>,
}

#[axum::debug_handler]
pub async fn authenticate(
  mut auth_session: AuthSession,
  Json(params): Json<AuthenticateQueryParams>,
) -> impl IntoResponse {
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

  let user = match auth_session
    .authenticate((email, UserSubmittedAuthCredentials::Password { password }))
    .await
  {
    Ok(Some(user)) => user,
    Ok(None) => {
      return (StatusCode::UNAUTHORIZED, Json(())).into_response();
    }
    Err(e) => {
      tracing::error!("failed to authenticate: {e}");
      return (StatusCode::INTERNAL_SERVER_ERROR, "internal error")
        .into_response();
    }
  };

  match auth_session.login(&user).await {
    Ok(_) => (StatusCode::OK, Json(user.id)).into_response(),
    Err(e) => {
      tracing::error!("failed to authenticate: {e}");
      (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
    }
  }
}
