use miette::{Context, IntoDiagnostic};
use models::{
  User,
  dvf::{EmailAddress, RecordId},
};
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthenticateParams {
  email:    String,
  password: String,
}

pub async fn authenticate(
  client: &Client,
  host: &Option<String>,
  port: &Option<u16>,
  email: &EmailAddress,
  password: &str,
) -> miette::Result<()> {
  tracing::debug!("authenticating as \"{email}\"");

  let params = AuthenticateParams {
    email:    email.clone().into_inner(),
    password: password.into(),
  };

  let req = client
    .post(format!(
      "http://{host}:{port}/authenticate",
      host = host.as_ref().cloned().unwrap_or("localhost".to_string()),
      port = port.unwrap_or(3000),
    ))
    .json(&params);

  let _user_id = req
    .send()
    .await
    .into_diagnostic()
    .context("failed to send authenticate request")?
    .error_for_status()
    .into_diagnostic()
    .context("authenticate request returned error")?
    .json::<RecordId<User>>()
    .await
    .into_diagnostic()
    .context("failed to read authenticate response")?;

  tracing::debug!("authenticated successfully");

  Ok(())
}
