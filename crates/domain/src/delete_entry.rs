use db::DatabaseError;
use models::{Entry, RecordId};

use crate::DomainService;

impl DomainService {
  /// Deletes an [`Entry`].
  #[tracing::instrument(skip(self))]
  pub async fn delete_entry(
    &self,
    id: RecordId<Entry>,
  ) -> Result<Entry, DatabaseError> {
    self.mutate.delete_entry(id).await
  }
}
