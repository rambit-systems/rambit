pub use paddle_rust_sdk_types::{
  entities::Subscription as PaddleSubscription,
  enums::SubscriptionStatus as PaddleSubscriptionStatus,
  ids::CustomerID as PaddleCustomerId,
};
use serde::{Deserialize, Serialize};

/// A client secret issued by Paddle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaddleClientSecret(pub String);

/// The Paddle environment being used.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PaddleEnvironment {
  /// A sandbox environment
  Sandbox,
  /// A production environment
  Production,
}

/// A report of an [`Org`]'s [`PaddleSubscription`]s.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgSubscriptionReport {
  /// The org's current subscription, if it exists.
  pub current: Option<PaddleSubscription>,
  /// The list of the org's past subscriptions.
  pub past:    Vec<PaddleSubscription>,
}
