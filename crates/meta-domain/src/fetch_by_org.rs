use db::{FetchModelByIndexError, kv::LaxSlug};
use models::{
  Cache, CacheIndexSelector, Entry, EntryIndexSelector, Org, Store,
  StoreIndexSelector, dvf::RecordId,
};

use crate::MetaService;

macro_rules! impl_fetch_by_org {
  (
    $(#[$meta:meta])*
    $vis:vis fn $method_name:ident($repo_field:ident) -> $model_ty:ident, $index_selector:ident
  ) => {
    $(#[$meta])*
    $vis async fn $method_name(
      &self,
      id: RecordId<Org>,
    ) -> Result<Vec<RecordId<$model_ty>>, FetchModelByIndexError> {
      self.$repo_field
        .fetch_ids_by_index(
          $index_selector::Org,
          LaxSlug::new(id.to_string()).into(),
        )
        .await
    }
  };
}

impl MetaService {
  impl_fetch_by_org!(
    /// Fetches all [`Cache`]s by org.
    pub fn fetch_caches_by_org(cache_repo) -> Cache, CacheIndexSelector
  );

  impl_fetch_by_org!(
    /// Fetches all [`Entry`]s by org.
    pub fn fetch_entries_by_org(entry_repo) -> Entry, EntryIndexSelector
  );

  impl_fetch_by_org!(
    /// Fetches all [`Store`]s by org.
    pub fn fetch_stores_by_org(store_repo) -> Store, StoreIndexSelector
  );
}
