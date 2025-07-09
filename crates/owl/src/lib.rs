//! Tools to manipulate NARs.

use belt::Belt;
use models::NarIntrensicData;

/// Interrogates a NAR and returns its intrensically known data.
pub struct NarInterrogator;

/// Possible failures of a NAR interrogation.
pub enum InterrogatorError {}

impl NarInterrogator {
  /// Interrogate a NAR and return its intrensically known data.
  pub async fn interrogate(
    data: Belt,
  ) -> Result<NarIntrensicData, InterrogatorError> {
    todo!()
  }
}
