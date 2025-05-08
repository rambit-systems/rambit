#![allow(dead_code)]

use dvf::{slugger::LaxSlug, FileSize};

/// A Nix store path digest.
pub struct PathDigest([u8; 20]);

/// A path in a Nix store.
pub struct StorePath {
  digest: PathDigest,
  name:   LaxSlug,
}

/// A path to a Nix derivation.
pub struct DerivationPath(StorePath);

/// A SHA-256 hash.
pub struct Sha256Hash([u8; 32]);

/// The bytes of a Nix NAR signature.
pub struct SignatureBytes(Box<[u8; 64]>);

/// A Nix NAR signature.
pub struct NarSignature {
  key_name: String,
  sig:      SignatureBytes,
}

/// The `.narinfo` data that describes a store path.
pub struct NarInfoStorePathDescription {
  /// Store path for this NAR.
  store_path: StorePath,
  /// Hash of the uncompressed NAR.
  nar_hash:   Sha256Hash,
  /// Size of the uncompressed NAR.
  nar_size:   FileSize,
  /// Store paths referred to by the contents of the NAR.
  references: Vec<StorePath>,
}

/// The `.narinfo` data that asserts authenticity.
pub struct NarInfoAuthenticityData {
  /// Signatures for this NAR.
  signatures: Vec<NarSignature>,
  // CAHash unimplemented
}

/// The `.narinfo` data about the deriver.
pub struct NarInfoDeriverData {
  /// The system string for this NAR's derivation.
  system:  Option<&'static str>,
  /// The path for this NAR's derivation.
  deriver: Option<DerivationPath>,
}

/// The `.narinfo` data specific to the binary cache.
pub struct NarInfoBinaryCacheData {
  /// URL for the NAR.
  url:         String,
  /// Compression algorithm used for the NAR.
  compression: Option<&'static str>,
  /// Hash of the compressed NAR.
  file_hash:   Option<Sha256Hash>,
  /// Size of the compressed NAR.
  file_size:   Option<FileSize>,
}
