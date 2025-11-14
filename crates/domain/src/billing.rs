//! Billing logic.

mod helpers;
mod subscriptions;

use models::{PaddleClientSecret, PaddleEnvironment};

use crate::DomainService;

impl DomainService {
  /// Return the Paddle client secret.
  pub fn paddle_client_secret(&self) -> PaddleClientSecret {
    self.billing.get_client_secret()
  }

  /// Returns the Paddle environment being used.
  pub fn paddle_environment(&self) -> PaddleEnvironment {
    self.billing.environment()
  }
}
