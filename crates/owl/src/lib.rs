//! Tools to manipulate NARs.

#![feature(slice_pattern)]

use core::slice::SlicePattern;
use std::{
  io::{self, BufRead, BufReader, Read},
  sync::LazyLock,
};

use belt::Belt;
use models::{NarIntrensicData, dvf::FileSize};
use nix_compat::store_path::StorePath;
use nix_nar::Content;
use regex::bytes::Regex as RegexBytes;

/// Interrogates a NAR and returns its intrensically known data.
pub struct NarInterrogator;

/// Possible failures of a NAR interrogation.
#[derive(Debug, thiserror::Error)]
pub enum InterrogatorError {
  /// Failed to read from input [`Belt`].
  #[error("Failed to read input data: {0}")]
  InputError(io::Error),
  /// Failed to decode NAR.
  #[error("Failed to decode NAR: {0}")]
  DecodingError(nix_nar::NarError),
}

static NIX_STORE_PATH_REGEX: LazyLock<RegexBytes> = LazyLock::new(|| {
  RegexBytes::new("(/nix/store/[a-z0-9]{32}-[a-zA-Z0-9._+?=-]{0,150})")
    .expect("failed to build regex engine")
});

impl NarInterrogator {
  /// Interrogate a NAR and return its intrensically known data.
  pub async fn interrogate(
    &self,
    data: Belt,
  ) -> Result<NarIntrensicData, InterrogatorError> {
    let buffered_data = data
      .collect()
      .await
      .map_err(InterrogatorError::InputError)?;
    let buffered_data_slice = buffered_data.as_slice();

    let nar_size = FileSize::new(
      buffered_data
        .len()
        .try_into()
        .expect("failed to convert from usize to u64"),
    );

    let nar_hash_string = sha256::digest(buffered_data_slice);
    let nar_hash: [u8; 32] = hex::decode(nar_hash_string)
      .expect("sha256 crate produced malformed hex string")
      .try_into()
      .expect("failed to squash sha256 digest bytes into 32 byte array");

    let decoder = nix_nar::Decoder::new(io::Cursor::new(buffered_data_slice))
      .map_err(InterrogatorError::DecodingError)?;
    let mut captures = Vec::new();
    for entry in decoder
      .entries()
      .map_err(InterrogatorError::DecodingError)?
    {
      let entry = entry.map_err(InterrogatorError::DecodingError)?;

      let Content::File { data, .. } = entry.content else {
        continue;
      };
      let mut entry_captures = extract_store_paths_from_reader(data)
        .expect("failed to read from memory-backed reader");
      captures.append(&mut entry_captures);
    }
    let references = captures
      .into_iter()
      .map(|p| {
        StorePath::<String>::from_absolute_path(p.as_bytes())
          .expect("failed to convert to store path")
      })
      .collect();

    Ok(NarIntrensicData {
      nar_hash,
      nar_size,
      references,
      ca_hash: None,
    })
  }
}

fn extract_store_paths_from_reader<R: Read>(
  reader: R,
) -> Result<Vec<String>, io::Error> {
  let re = &NIX_STORE_PATH_REGEX;
  let mut buf_reader = BufReader::new(reader);

  let mut captures = Vec::new();

  loop {
    let mut line = Vec::new();
    match buf_reader.read_until(b'\n', &mut line) {
      Ok(0) => break, // EOF
      Ok(_) => {
        let line_captures = re.captures_iter(&line).map(|c| {
          c.get(0)
            .expect("no zeroth capture group")
            .as_bytes()
            .utf8_chunks()
            .map(|c| c.valid())
            .collect::<Vec<_>>()
            .join("")
        });
        captures.extend(line_captures);
      }
      Err(e) => return Err(e),
    }
  }

  Ok(captures)
}

#[cfg(test)]
mod test {
  use belt::Belt;

  use crate::NarInterrogator;

  #[tokio::test]
  async fn test_bat_nar() {
    let bat_nar =
      include_bytes!("../test/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0");

    let interrogator = NarInterrogator;
    let data = interrogator
      .interrogate(Belt::from_bytes(
        bytes::Bytes::from(bat_nar.as_slice()),
        None,
      ))
      .await
      .unwrap();

    println!("{data:?}");
    println!(
      "{:?}",
      data
        .references
        .iter()
        .map(|p| p.to_absolute_path())
        .collect::<Vec<_>>()
    );
  }
}
