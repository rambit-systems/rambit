use db::DatabaseError;
use models::{Entry, RecordId};

use super::MutationService;

impl MutationService {
  /// Deletes an [`Entry`].
  #[tracing::instrument(skip(self))]
  pub async fn delete_entry(
    &self,
    id: RecordId<Entry>,
  ) -> Result<Entry, DatabaseError> {
    self.entry_repo.delete_and_return(id).await
  }
}
