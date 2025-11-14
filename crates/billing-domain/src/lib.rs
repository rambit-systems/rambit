//! Entrypoint for logic in the billing domain.

mod customer;
mod subscription;

use std::sync::Arc;

use miette::{Context, IntoDiagnostic, Report};
use models::{PaddleClientSecret, PaddleEnvironment};
use paddle_rust_sdk::Paddle;

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

  /// Returns the Paddle client secret.
  pub fn get_client_secret(&self) -> PaddleClientSecret {
    self.client_secret.clone()
  }

  /// Returns the Paddle environment being used.
  pub fn environment(&self) -> PaddleEnvironment { self.environment }
}
