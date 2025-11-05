use db::DatabaseError;
use models::{
  Cache, CacheIndexSelector, EntityName, Org, OrgIdent, OrgIndexSelector,
  RecordId, Store, StoreIndexSelector, model::IndexValue,
};

use crate::MetaService;

impl MetaService {
  /// Fetches a [`Cache`] by its [name](CacheIndexSelector::Name).
  #[tracing::instrument(skip(self))]
  pub async fn fetch_cache_by_name(
    &self,
    name: EntityName,
  ) -> Result<Option<Cache>, DatabaseError> {
    self
      .cache_repo
      .find_by_unique_index(
        CacheIndexSelector::Name,
        &IndexValue::new_single(name.into_inner()),
      )
      .await
  }

  /// Fetches a [`Store`] by its org and name.
  #[tracing::instrument(skip(self))]
  pub async fn fetch_store_by_org_and_name(
    &self,
    org: RecordId<Org>,
    store_name: EntityName,
  ) -> Result<Option<Store>, DatabaseError> {
    self
      .store_repo
      .find_by_unique_index(
        StoreIndexSelector::NameByOrg,
        &Store::unique_index_name_by_org(org, &store_name),
      )
      .await
  }

  /// Fetches an [`Org`] by its [`OrgIdent`].
  #[tracing::instrument(skip(self))]
  pub async fn fetch_org_by_ident(
    &self,
    org_ident: OrgIdent,
  ) -> Result<Option<Org>, DatabaseError> {
    self
      .org_repo
      .find_by_unique_index(OrgIndexSelector::Ident, &org_ident.index_value())
      .await
  }
}
