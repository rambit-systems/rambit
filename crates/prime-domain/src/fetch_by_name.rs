use db::FetchModelByIndexError;
use models::{Cache, CacheUniqueIndexSelector, dvf::EntityName};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Fetches a [`Cache`] by its [name](CacheUniqueIndexSelector::Name).
  pub async fn fetch_cache_by_name(
    &self,
    name: EntityName,
  ) -> Result<Option<Cache>, FetchModelByIndexError> {
    self
      .cache_repo
      .fetch_model_by_unique_index(
        CacheUniqueIndexSelector::Name,
        name.into_inner().into(),
      )
      .await
  }
}
