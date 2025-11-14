pub use paddle_rust_sdk_types::{
  entities::Subscription as PaddleSubscription,
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
