use db::DatabaseError;
use models::{
  Cache, Digest, Entry, EntryIndexSelector, RecordId, Store, StorePath,
};

use super::MetaService;

impl MetaService {
  /// Fetches a [`Entry`] by its
  /// [cache-id-and-entry-digest](EntryIndexSelector::CacheIdAndEntryDigest).
  #[tracing::instrument(skip(self))]
  pub async fn fetch_entry_by_cache_id_and_entry_digest(
    &self,
    cache_id: RecordId<Cache>,
    entry_digest: Digest,
  ) -> Result<Option<Entry>, DatabaseError> {
    self
      .entry_repo
      .find_by_unique_index(
        EntryIndexSelector::CacheIdAndEntryDigest,
        &Entry::unique_index_cache_id_and_entry_digest_single(
          cache_id,
          entry_digest,
        ),
      )
      .await
  }

  /// Fetches a [`Entry`] by its
  /// [store-id-and-entry-path](EntryIndexSelector::CacheIdAndEntryDigest).
  #[tracing::instrument(skip(self))]
  pub async fn fetch_entry_by_store_id_and_entry_path(
    &self,
    store_id: RecordId<Store>,
    entry_path: &StorePath<String>,
  ) -> Result<Option<Entry>, DatabaseError> {
    self
      .entry_repo
      .find_by_unique_index(
        EntryIndexSelector::StoreIdAndEntryPath,
        &Entry::unique_index_store_id_and_entry_path(store_id, entry_path),
      )
      .await
  }
}
