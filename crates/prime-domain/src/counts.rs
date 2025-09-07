use db::{FetchModelByIndexError, kv::LaxSlug};
use models::{Cache, EntryIndexSelector, Store, dvf::RecordId};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Counts the number of [`Entry`](models::Entry)s in a [`Store`].
  pub async fn count_entries_in_store(
    &self,
    store: RecordId<Store>,
  ) -> Result<u32, FetchModelByIndexError> {
    self
      .entry_repo
      .count_models_by_index(
        EntryIndexSelector::Store,
        LaxSlug::new(store.to_string()).into(),
      )
      .await
  }

  /// Counts the number of [`Entry`](models::Entry)s in a [`Cache`].
  pub async fn count_entries_in_cache(
    &self,
    cache: RecordId<Cache>,
  ) -> Result<u32, FetchModelByIndexError> {
    self
      .entry_repo
      .count_models_by_index(
        EntryIndexSelector::Cache,
        LaxSlug::new(cache.to_string()).into(),
      )
      .await
  }
}
