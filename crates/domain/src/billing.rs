use models::{
  Org, PaddleClientSecret, PaddleEnvironment, PaddleSubscription, RecordId,
};

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

  /// Gets the Paddle subscriptions for a given [`Org`].
  pub fn get_subscriptions_for_org(
    &self,
    org_id: RecordId<Org>,
  ) -> miette::Result<Vec<PaddleSubscription>> {
    todo!()
  }
}
