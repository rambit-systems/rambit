use std::{fmt, str::FromStr};

use nix_compat::{nixbase32, store_path::DIGEST_SIZE};
use serde::{Deserialize, Serialize};

/// A store path digest.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Digest([u8; DIGEST_SIZE]);

impl Digest {
  /// Provides access to the inner buffer.
  pub fn inner(&self) -> &[u8; DIGEST_SIZE] { &self.0 }

  /// Creates a digest from its bytes.
  pub fn from_bytes(input: [u8; DIGEST_SIZE]) -> Self { Self(input) }
}

impl fmt::Display for Digest {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", nixbase32::encode(&self.0))
  }
}

impl FromStr for Digest {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(nixbase32::decode_fixed(s.as_bytes()).map_err(|_| ())?))
  }
}
