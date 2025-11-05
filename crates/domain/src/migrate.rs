use miette::Result;

use crate::DomainService;

impl DomainService {
  /// Add test data to databases.
  pub async fn migrate_test_data(&self) -> Result<()> {
    self.mutate.migrate_test_data().await
  }
}
