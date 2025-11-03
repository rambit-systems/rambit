use std::fmt;

use serde::{Deserialize, Serialize};
use slug::Slug;

/// An entity name using [`Slug`] rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntityName(Slug);

impl EntityName {
  /// Create an [`EntityName`] from a string-like. Sanitizes according to
  /// [`Slug`] rules.
  #[must_use]
  pub fn new(input: impl AsRef<str>) -> Self { Self(Slug::new(input.as_ref())) }

  /// Get the inner [`Slug`].
  #[must_use]
  pub fn into_inner(self) -> Slug { self.0 }
}

impl fmt::Display for EntityName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl AsRef<str> for EntityName {
  fn as_ref(&self) -> &str { &self.0 }
}
