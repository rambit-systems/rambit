use db::{FetchModelByIndexError, kv::LaxSlug};
use models::{
  Cache, CacheUniqueIndexSelector, Org, OrgIdent, OrgUniqueIndexSelector,
  Store, StoreUniqueIndexSelector,
  dvf::{EntityName, RecordId},
};

use crate::MetaService;

impl MetaService {
  /// Fetches a [`Cache`] by its [name](CacheUniqueIndexSelector::Name).
  #[tracing::instrument(skip(self))]
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

  /// Fetches a [`Store`] by its org and name.
  #[tracing::instrument(skip(self))]
  pub async fn fetch_store_by_org_and_name(
    &self,
    org: RecordId<Org>,
    store_name: EntityName,
  ) -> Result<Option<Store>, FetchModelByIndexError> {
    self
      .store_repo
      .fetch_model_by_unique_index(
        StoreUniqueIndexSelector::NameByOrg,
        LaxSlug::new(format!("{org}-{store_name}")).into(),
      )
      .await
  }

  /// Fetches an [`Org`] by its [`OrgIdent`].
  #[tracing::instrument(skip(self))]
  pub async fn fetch_org_by_ident(
    &self,
    org_ident: OrgIdent,
  ) -> Result<Option<Org>, FetchModelByIndexError> {
    self
      .org_repo
      .fetch_model_by_unique_index(
        OrgUniqueIndexSelector::Ident,
        org_ident.index_value().into(),
      )
      .await
  }
}
