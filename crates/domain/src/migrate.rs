use miette::Result;

use crate::DomainService;

impl DomainService {
  /// Add test data to databases.
  pub async fn migrate_test_data(&self, ephemeral_storage: bool) -> Result<()> {
    self.mutate.migrate_test_data(ephemeral_storage).await
  }
}
