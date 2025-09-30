use db::DeleteModelError;
use models::{Entry, dvf::RecordId};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Deletes an [`Entry`].
  pub async fn delete_entry(
    &self,
    id: RecordId<Entry>,
  ) -> Result<Option<RecordId<Entry>>, DeleteModelError> {
    self
      .entry_repo
      .delete_model(id)
      .await
      .map(|b| b.then_some(id))
  }
}
