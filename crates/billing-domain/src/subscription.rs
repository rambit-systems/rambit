use miette::Diagnostic;
use models::{PaddleCustomerId, PaddleSubscription};

use crate::BillingService;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum SubscriptionsForCustomerError {
  /// An error returned by Paddle.
  #[error("paddle error: {0}")]
  PaddleError(#[from] paddle_rust_sdk::Error),
}

impl BillingService {
  /// Gets the subscriptions attached to a customer.
  pub async fn get_all_subscriptions_for_customer(
    &self,
    customer_id: &PaddleCustomerId,
  ) -> Result<Vec<PaddleSubscription>, SubscriptionsForCustomerError> {
    let mut req = self.paddle_client.subscriptions_list();
    req.customer_id([customer_id.clone()]);
    let mut paginator = req.send();

    let mut results = Vec::new();
    while let Some(page) = paginator.next().await? {
      results.extend_from_slice(&page.data[..]);
    }

    Ok(results)
  }
}
