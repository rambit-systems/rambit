use nix_compat::store_path::{ENCODED_DIGEST_SIZE, StorePath};

/// Abbreviate an item.
pub trait Abbreviate {
  /// Abbreviate an item.
  fn abbreviate(&self) -> String;
}

impl<S: AsRef<str>> Abbreviate for StorePath<S> {
  fn abbreviate(&self) -> String {
    const COUNT: usize = 4;
    let string = self.to_string();
    let separator_index =
      string.find('-').expect("no separator found in store path");
    let (digest, rest) = string.split_at(separator_index);
    format!(
      "{first}…{last}{rest}",
      first = &digest[0..COUNT],
      last = &digest[ENCODED_DIGEST_SIZE - COUNT..ENCODED_DIGEST_SIZE]
    )
  }
}
