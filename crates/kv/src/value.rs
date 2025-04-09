//! Value type for key-value store.

use serde::{de::DeserializeOwned, Serialize};

/// Represents a value in a key-value store.
#[derive(Debug, Clone, PartialEq)]
pub struct Value(Vec<u8>);

impl Value {
  /// Create a new value with the given bytes.
  pub fn new(value: Vec<u8>) -> Self { Self(value) }
  /// Get the inner bytes of the value.
  pub fn into_inner(self) -> Vec<u8> { self.0 }

  /// Serialize a value into a [`Value`], using MessagePack.
  pub fn serialize<T: Serialize>(
    value: &T,
  ) -> Result<Self, rmp_serde::encode::Error> {
    Ok(Self(rmp_serde::to_vec_named(value)?))
  }
  /// Deserialize a value from a [`Value`], using MessagePack.
  pub fn deserialize<T: DeserializeOwned>(
    self,
  ) -> Result<T, rmp_serde::decode::Error> {
    rmp_serde::from_read(self.0.as_slice())
  }
}

impl From<Value> for Vec<u8> {
  fn from(value: Value) -> Self { value.0 }
}

impl From<Vec<u8>> for Value {
  fn from(value: Vec<u8>) -> Self { Self(value) }
}

impl From<&[u8]> for Value {
  fn from(value: &[u8]) -> Self { Self(value.to_vec()) }
}

impl From<&str> for Value {
  fn from(value: &str) -> Self { Self(value.as_bytes().to_vec()) }
}

impl AsRef<[u8]> for Value {
  fn as_ref(&self) -> &[u8] { &self.0 }
}

#[cfg(feature = "redb")]
mod redb {
  use std::any::type_name;

  use super::Value;

  impl redb::Value for Value {
    type SelfType<'a>
      = Value
    where
      Self: 'a;

    type AsBytes<'a>
      = Value
    where
      Self: 'a;

    fn fixed_width() -> Option<usize> { None }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
      Self: 'a,
    {
      Self::from(data)
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
      Self: 'b,
    {
      value.clone()
    }

    fn type_name() -> redb::TypeName {
      redb::TypeName::new(type_name::<Value>())
    }
  }
}
