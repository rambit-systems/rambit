use db::FetchModelByIndexError;
use models::{
  Cache, Digest, Entry, EntryUniqueIndexSelector, Store, StorePath,
  dvf::RecordId,
};

use super::MetaService;

impl MetaService {
  /// Fetches a [`Entry`] by its
  /// [cache-id-and-entry-digest](EntryUniqueIndexSelector::CacheIdAndEntryDigest).
  #[tracing::instrument(skip(self))]
  pub async fn fetch_entry_by_cache_id_and_entry_digest(
    &self,
    cache_id: RecordId<Cache>,
    entry_digest: Digest,
  ) -> Result<Option<Entry>, FetchModelByIndexError> {
    self
      .entry_repo
      .fetch_model_by_unique_index(
        EntryUniqueIndexSelector::CacheIdAndEntryDigest,
        Entry::unique_index_cache_id_and_entry_digest(cache_id, entry_digest),
      )
      .await
  }

  /// Fetches a [`Entry`] by its
  /// [store-id-and-entry-path](EntryUniqueIndexSelector::CacheIdAndEntryDigest).
  #[tracing::instrument(skip(self))]
  pub async fn fetch_entry_by_store_id_and_entry_path(
    &self,
    store_id: RecordId<Store>,
    entry_path: &StorePath<String>,
  ) -> Result<Option<Entry>, FetchModelByIndexError> {
    self
      .entry_repo
      .fetch_model_by_unique_index(
        EntryUniqueIndexSelector::StoreIdAndEntryPath,
        Entry::unique_index_store_id_and_entry_path(store_id, entry_path),
      )
      .await
  }
}
