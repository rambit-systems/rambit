use std::fmt;

use serde::{Deserialize, Serialize};

/// An entity's visibility.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Visibility {
  /// The entity is public.
  Public,
  /// The entity is private.
  Private,
}

impl fmt::Display for Visibility {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Visibility::Public => write!(f, "Public"),
      Visibility::Private => write!(f, "Private"),
    }
  }
}
