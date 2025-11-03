use std::str::FromStr;
#[cfg(feature = "reasonable-email")]
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[cfg(feature = "reasonable-email")]
/// Our "reasonable" email regex is significantly more restrictive than the
/// RFCs. sourced from https://colinhacks.com/essays/reasonable-email-regex,
/// with lowercase ranges added.
const EMAIL_REASONABLE_REGEX_STRING: &str = r#"^([A-Za-z0-9_+-]+\.?)*[A-Za-z0-9_+-]@([A-Za-z0-9][A-Za-z0-9-]*\.)+[A-Za-z]{2,}$"#;
#[cfg(feature = "reasonable-email")]
static EMAIL_REASONABLE_REGEX: LazyLock<regex::Regex> =
  LazyLock::new(|| regex::Regex::new(EMAIL_REASONABLE_REGEX_STRING).unwrap());

/// Validate that an email address is technically compliant.
pub fn validate_compliant_email_address(email: &str) -> bool {
  email_address::EmailAddress::from_str(email).is_ok()
}

#[cfg(feature = "reasonable-email")]
/// Validate that an email address is reasonable.
pub fn validate_reasonable_email_address(email: &str) -> bool {
  EMAIL_REASONABLE_REGEX.is_match(email)
}

/// An email address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailAddress(String);

/// Error for [`EmailAddress::try_new()`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EmailAddressError {
  /// The email is invalid.
  #[error("The email is invalid.")]
  InvalidEmail,
  /// The email is too long.
  #[error("The email is too long.")]
  TooLong,
}

impl EmailAddress {
  /// Create a new EmailAddress with validation.
  pub fn try_new(email: String) -> Result<Self, EmailAddressError> {
    if email.chars().count() > 128 {
      return Err(EmailAddressError::TooLong);
    }
    if !validate_compliant_email_address(&email) {
      return Err(EmailAddressError::InvalidEmail);
    }
    Ok(Self(email))
  }

  /// Get the email address as a string slice.
  pub fn as_str(&self) -> &str { &self.0 }

  #[cfg(feature = "reasonable-email")]
  /// Check if the email address is reasonable.
  pub fn is_reasonable(&self) -> bool {
    validate_reasonable_email_address(self.as_ref())
  }
}

impl AsRef<str> for EmailAddress {
  fn as_ref(&self) -> &str { &self.0 }
}

impl std::fmt::Display for EmailAddress {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl FromStr for EmailAddress {
  type Err = EmailAddressError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::try_new(s.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compliance_check() {
    let reasonable_email = "bob@example.com";
    let compliant_email = "main@[192.168.0.1]";
    assert!(validate_compliant_email_address(compliant_email));
    assert!(validate_compliant_email_address(reasonable_email));
  }

  #[test]
  #[cfg(feature = "reasonable-email")]
  fn test_reasonable_check() {
    let reasonable_email = "bob@example.com";
    let compliant_email = "main@[192.168.0.1]";
    assert!(!validate_reasonable_email_address(compliant_email));
    assert!(validate_reasonable_email_address(reasonable_email));
  }
}
