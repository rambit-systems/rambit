use db::{FetchModelByIndexError, kv::LaxSlug};
use models::{
  Cache, CacheIndexSelector, Entry, EntryIndexSelector, Org, Store,
  StoreIndexSelector, dvf::RecordId,
};

use crate::PrimeDomainService;

macro_rules! impl_fetch_by_org {
  (
    $( #[$meta:meta] )*
    $vis:vis fn $method_name:ident,
    repo: $repo_field:ident,
    model: $model_ty:ty,
    index_selector: $index_selector:ty
  ) => {
    $( #[$meta] )*
    $vis async fn $method_name(
      &self,
      id: RecordId<Org>,
    ) -> Result<Vec<$model_ty>, FetchModelByIndexError> {
      self.$repo_field
        .fetch_model_by_index(
          <$index_selector>::Org,
          LaxSlug::new(id.to_string()).into(),
        )
        .await
    }
  };
}

impl PrimeDomainService {
  impl_fetch_by_org!(
    /// Fetches all [`Cache`]s by org.
    pub fn fetch_caches_by_org,
    repo: cache_repo,
    model: Cache,
    index_selector: CacheIndexSelector
  );

  impl_fetch_by_org!(
    /// Fetches all [`Entry`]s by org.
    pub fn fetch_entries_by_org,
    repo: entry_repo,
    model: Entry,
    index_selector: EntryIndexSelector
  );
}
