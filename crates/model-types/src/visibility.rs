use std::{fmt, str::FromStr};

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

impl FromStr for Visibility {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().to_lowercase().as_ref() {
      "public" => Ok(Visibility::Public),
      "private" => Ok(Visibility::Private),
      _ => Err(()),
    }
  }
}
