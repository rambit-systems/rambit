//! Entrypoint for logic in the billing domain.

use std::{collections::HashMap, sync::Arc};

use miette::{Context, IntoDiagnostic, Report};
use models::{
  EmailAddress, PaddleClientSecret, PaddleCustomerId, PaddleEnvironment,
  RecordId, User,
};
use paddle_rust_sdk::{Paddle, error::PaddleApiError, response::ErrorResponse};

/// Entrypoint for logic in the billing domain.
#[derive(Clone, Debug)]
pub struct BillingService {
  paddle_client: Arc<Paddle>,
  client_secret: PaddleClientSecret,
  environment:   PaddleEnvironment,
}

impl BillingService {
  /// Create a new [`BillingService`].
  pub fn new(
    api_key: &str,
    client_secret: &str,
    environment: PaddleEnvironment,
  ) -> Result<Self, Report> {
    let url = match environment {
      PaddleEnvironment::Sandbox => Paddle::SANDBOX,
      PaddleEnvironment::Production => Paddle::PRODUCTION,
    };

    miette::ensure!(!api_key.is_empty(), "paddle api key is empty");
    miette::ensure!(!client_secret.is_empty(), "paddle client secret is empty");

    Ok(Self {
      paddle_client: Arc::new(
        Paddle::new(api_key, url)
          .into_diagnostic()
          .context("failed to initialize paddle client")?,
      ),
      client_secret: PaddleClientSecret(client_secret.to_owned()),
      environment,
    })
  }

  /// Create a new [`BillingService`] from environment variables.
  pub fn new_from_env() -> Result<Self, Report> {
    let api_key = std::env::var("PADDLE_API_KEY")
      .into_diagnostic()
      .context("failed to read var `PADDLE_API_KEY`")?;

    let client_secret = std::env::var("PADDLE_CLIENT_KEY")
      .into_diagnostic()
      .context("failed to read var `PADDLE_CLIENT_KEY`")?;

    let is_sandbox = std::env::var("PADDLE_SANDBOX")
      .map(|v| !v.is_empty() && v != "0" && v != "false")
      .unwrap_or(false);

    let environment = match is_sandbox {
      true => PaddleEnvironment::Sandbox,
      false => PaddleEnvironment::Production,
    };

    Self::new(&api_key, &client_secret, environment)
  }
}

impl BillingService {
  /// Returns the Paddle client secret.
  pub fn get_client_secret(&self) -> PaddleClientSecret {
    self.client_secret.clone()
  }

  /// Returns the Paddle environment being used.
  pub fn environment(&self) -> PaddleEnvironment { self.environment }

  /// Creates a new customer if a customer with the given email does not already
  /// exist. Otherwise, update the ID and name of the customer whose email
  /// matches.
  pub async fn upsert_customer(
    &self,
    org_id: RecordId<User>,
    name: &str,
    email: &EmailAddress,
  ) -> Result<PaddleCustomerId, Report> {
    // attempt to just create a user
    let mut req = self.paddle_client.customer_create(email.as_ref());
    req
      .name(name)
      .custom_data(HashMap::from_iter([("id".to_owned(), org_id.to_string())]));
    let create_result = req.send().await;

    // short circuit if it worked
    let err = match create_result {
      Ok(customer) => return Ok(customer.data.id),
      Err(e) => e,
    };

    // extract the ID if it's a duplicate customer error
    let id = match err {
      paddle_rust_sdk::Error::PaddleApi(ErrorResponse {
        error: PaddleApiError { code, detail, .. },
        ..
      }) if code == "customer_already_exists" => detail
        .split(" ")
        .last()
        .ok_or(miette::miette!(
          "could not find customer ID in duplicate customer error: {detail:?}"
        ))?
        .to_owned(),
      e => {
        return Err(
          Report::from_err(e)
            .context("unknown paddle error in attempted customer creation"),
        );
      }
    };

    // update the customer name and Rambit ID, and activate if archived
    let mut update_req = self.paddle_client.customer_update(&*id);
    update_req
      .name(name)
      .custom_data(HashMap::from_iter([("id".to_owned(), org_id.to_string())]))
      .status(paddle_rust_sdk::enums::Status::Active);

    let customer = update_req
      .send()
      .await
      .into_diagnostic()
      .context("failed to update paddle customer")?
      .data;

    Ok(customer.id)
  }
}
