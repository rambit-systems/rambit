//! User mutation logic.

use db::PatchModelError;
use models::User;

use super::MutationService;

impl MutationService {
  /// Patches a [`User`].
  pub async fn patch_user(&self, user: User) -> Result<User, PatchModelError> {
    self.user_repo.patch_model(user.id, user).await
  }
}
