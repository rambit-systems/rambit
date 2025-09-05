use db::CreateModelError;
use models::{
  Cache, Org,
  dvf::{EntityName, RecordId, Visibility},
};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Creates a [`Cache`].
  pub async fn create_cache(
    &self,
    org: RecordId<Org>,
    name: EntityName,
    visibility: Visibility,
  ) -> Result<RecordId<Cache>, CreateModelError> {
    self
      .cache_repo
      .create_model(Cache {
        id: RecordId::new(),
        org,
        name,
        visibility,
      })
      .await
      .map(|c| c.id)
  }
}
