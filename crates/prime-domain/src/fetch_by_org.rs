use db::{FetchModelByIndexError, kv::LaxSlug};
use models::{
  Cache, CacheIndexSelector, Entry, EntryIndexSelector, Org, dvf::RecordId,
};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Fetches all [`Cache`]s by org.
  pub async fn fetch_cache_by_org(
    &self,
    id: RecordId<Org>,
  ) -> Result<Vec<Cache>, FetchModelByIndexError> {
    self
      .cache_repo
      .fetch_model_by_index(
        CacheIndexSelector::Org,
        LaxSlug::new(id.to_string()).into(),
      )
      .await
  }

  /// Fetches all [`Entry`]s by org.
  pub async fn fetch_entries_by_org(
    &self,
    id: RecordId<Org>,
  ) -> Result<Vec<Entry>, FetchModelByIndexError> {
    self
      .entry_repo
      .fetch_model_by_index(
        EntryIndexSelector::Org,
        LaxSlug::new(id.to_string()).into(),
      )
      .await
  }
}
