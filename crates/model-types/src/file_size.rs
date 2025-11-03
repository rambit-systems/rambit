use std::fmt;

use serde::{Deserialize, Serialize};

/// The size of a file.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FileSize(u64);

impl AsRef<u64> for FileSize {
  fn as_ref(&self) -> &u64 { &self.0 }
}

impl fmt::Display for FileSize {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    const KB: u64 = 1000;
    const MB: u64 = 1000 * KB;
    const GB: u64 = 1000 * MB;

    match *self.as_ref() {
      size if size < KB => write!(f, "{size} B"),
      size if size < MB => write!(f, "{:.2} KB", size as f64 / KB as f64),
      size if size < GB => {
        write!(f, "{:.2} MB", size as f64 / MB as f64)
      }
      size => write!(f, "{:.2} GB", size as f64 / GB as f64),
    }
  }
}

impl fmt::Debug for FileSize {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("FileSize").field(&self.to_string()).finish()
  }
}
