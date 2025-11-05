//! User mutation logic.

use db::DatabaseError;
use models::User;

use super::MutationService;

impl MutationService {
  /// Patches a [`User`].
  #[tracing::instrument(skip(self))]
  pub async fn patch_user(&self, user: &User) -> Result<(), DatabaseError> {
    self.user_repo.update(user).await
  }
}
