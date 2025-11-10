use serde::{Deserialize, Serialize};

/// A customer ID received from Paddle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaddleCustomerId(String);

impl PaddleCustomerId {
  /// Return the inner [`String`].
  pub fn into_inner(self) -> String { self.0 }
}

impl AsRef<str> for PaddleCustomerId {
  fn as_ref(&self) -> &str { &self.0 }
}
