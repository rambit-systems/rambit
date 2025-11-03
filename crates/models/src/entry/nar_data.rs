use std::{collections::HashSet, path::PathBuf};

use model::RecordId;
use model_types::{CompressionStatus, FileSize};
use nix_compat::{narinfo::Signature, nixhash::CAHash, store_path::StorePath};
use serde::{Deserialize, Serialize};

use crate::Store;

/// Data intrensic to the NAR contents of an [`Entry`](super::Entry). This can
/// all be derived from the NAR file itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NarIntrensicData {
  /// The SHA-256 digest of the NAR file.
  pub nar_hash:   [u8; 32],
  /// The size of the NAR file.
  pub nar_size:   FileSize,
  /// Store paths known to be referenced by the contents.
  pub references: HashSet<StorePath<String>>,
  /// The content-addressed hash of the entry.
  pub ca_hash:    Option<CAHash>,
}

/// Data about how the NAR exists in the [`Store`](crate::Store).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NarStorageData {
  /// The entry's store.
  pub store:              RecordId<Store>,
  /// The path to the entry's data within the store.
  pub storage_path:       PathBuf,
  /// The compression status of the entry data in-situ within its
  /// [`Store`](crate::Store).
  pub compression_status: CompressionStatus,
}

/// Authenticity data for an entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NarAuthenticityData {
  /// The signatures on the NAR's fingerprint data.
  pub signatures: Vec<Signature<String>>,
}

/// Data about the NAR's deriver.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NarDeriverData {
  /// The system triple of the deriver field.
  pub system:  Option<String>,
  /// The store path of the derivation that produced the store entry. The last
  /// .drv suffix is stripped.
  pub deriver: Option<StorePath<String>>,
}
