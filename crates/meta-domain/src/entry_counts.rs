use db::DatabaseError;
use models::{Cache, EntryIndexSelector, RecordId, Store, model::IndexValue};

use super::MetaService;

impl MetaService {
  /// Counts the number of [`Entry`](models::Entry)s in a [`Store`].
  #[tracing::instrument(skip(self))]
  pub async fn count_entries_in_store(
    &self,
    store: RecordId<Store>,
  ) -> Result<u64, DatabaseError> {
    self
      .entry_repo
      .count_by_index(
        EntryIndexSelector::Store,
        &IndexValue::new_single(store.to_string()),
      )
      .await
  }

  /// Counts the number of [`Entry`](models::Entry)s in a [`Cache`].
  #[tracing::instrument(skip(self))]
  pub async fn count_entries_in_cache(
    &self,
    cache: RecordId<Cache>,
  ) -> Result<u64, DatabaseError> {
    self
      .entry_repo
      .count_by_index(
        EntryIndexSelector::Caches,
        &IndexValue::new_single(cache.to_string()),
      )
      .await
  }
}
