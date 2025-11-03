use std::fmt;

use serde::{Deserialize, Serialize};

/// The maximum length for a [`HumanName`].
pub const HUMAN_NAME_MAX_LENGTH: usize = 255;

/// A human's name. Only constrains length: `0 < len <=
/// `[`HUMAN_NAME_MAX_LENGTH`]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HumanName(String);

/// Error for [`HumanName::try_new()`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HumanNameError {
  /// The name is empty.
  #[error("The name is empty.")]
  Empty,
  /// The name is too long.
  #[error("The name is too long.")]
  TooLong,
}

impl HumanName {
  /// Create a [`HumanName`] from a string-like. Constrains length to `0 < len
  /// <= `[`HUMAN_NAME_MAX_LENGTH`].
  pub fn try_new(input: impl AsRef<str>) -> Result<Self, HumanNameError> {
    let input = input.as_ref();
    match input.len() {
      0 => Err(HumanNameError::Empty),
      l if l > HUMAN_NAME_MAX_LENGTH => Err(HumanNameError::TooLong),
      _ => Ok(Self(input.to_owned())),
    }
  }

  /// Get the inner [`String`].
  #[must_use]
  pub fn into_inner(self) -> String { self.0 }
}

impl fmt::Display for HumanName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl AsRef<str> for HumanName {
  fn as_ref(&self) -> &str { &self.0 }
}
