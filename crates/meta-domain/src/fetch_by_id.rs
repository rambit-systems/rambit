use db::FetchModelError;
use models::{Cache, Entry, Org, Store, User, dvf::RecordId};

use super::MetaService;

macro_rules! impl_fetch_by_id {
  ($($method:ident, $model:ty, $repo_field:ident);* $(;)?) => {
    $(
      #[doc = concat!("Fetches a [`", stringify!($model), "`] by its ID.")]
      #[tracing::instrument(skip(self))]
      pub async fn $method(
        &self,
        id: RecordId<$model>,
      ) -> Result<Option<$model>, FetchModelError> {
        tracing::warn!("fetching {} by id {}", stringify!($model), id);
        self.$repo_field.fetch_model_by_id(id).await
      }
    )*
  };
}

impl MetaService {
  impl_fetch_by_id! {
    fetch_org_by_id, Org, org_repo;
    fetch_user_by_id, User, user_repo;
    fetch_store_by_id, Store, store_repo;
    fetch_entry_by_id, Entry, entry_repo;
    fetch_cache_by_id, Cache, cache_repo;
  }
}
