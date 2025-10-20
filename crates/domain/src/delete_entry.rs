use db::DeleteModelError;
use models::{Entry, dvf::RecordId};

use crate::DomainService;

impl DomainService {
  /// Deletes an [`Entry`].
  #[tracing::instrument(skip(self))]
  pub async fn delete_entry(
    &self,
    id: RecordId<Entry>,
  ) -> Result<Option<RecordId<Entry>>, DeleteModelError> {
    self.mutate.delete_entry(id).await
  }
}
