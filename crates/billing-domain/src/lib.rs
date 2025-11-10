//! Entrypoint for logic in the billing domain.

use std::{collections::HashMap, sync::Arc};

use miette::{Context, IntoDiagnostic, Report};
use models::{EmailAddress, Org, PaddleCustomerId, RecordId};
use paddle_rust_sdk::Paddle;

/// Entrypoint for logic in the billing domain.
#[derive(Clone, Debug)]
pub struct BillingService {
  paddle_client: Arc<Paddle>,
}

impl BillingService {
  /// Create a new [`BillingDomain`].
  pub fn new(api_key: &str, is_sandbox: bool) -> Result<Self, Report> {
    let url = if is_sandbox {
      Paddle::SANDBOX
    } else {
      Paddle::PRODUCTION
    };
    Ok(Self {
      paddle_client: Arc::new(
        Paddle::new(api_key, url)
          .into_diagnostic()
          .context("failed to initialize paddle client")?,
      ),
    })
  }

  /// Create a new [`BillingDomain`] from environment variables.
  pub fn new_from_env() -> Result<Self, Report> {
    let api_key = std::env::var("PADDLE_API_KEY")
      .into_diagnostic()
      .context("failed to read var `PADDLE_API_KEY`")?;
    let is_sandbox = std::env::var("PADDLE_SANDBOX")
      .map(|v| !v.is_empty() && v != "0" && v != "false")
      .unwrap_or(false);
    Self::new(&api_key, is_sandbox)
  }
}

impl BillingService {
  /// Create a new customer.
  pub async fn create_customer(
    &self,
    id: RecordId<Org>,
    name: &str,
    email: &EmailAddress,
  ) -> Result<PaddleCustomerId, Report> {
    let mut req = self.paddle_client.customer_create(email.as_ref());

    req
      .name(name)
      .custom_data(HashMap::from_iter([("id".to_owned(), id.to_string())]));

    let customer = req
      .send()
      .await
      .into_diagnostic()
      .context("failed to create paddle customer")?
      .data;

    Ok(PaddleCustomerId::new(customer.id.0))
  }
}
