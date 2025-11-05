use db::DatabaseError;
use models::{EmailAddress, User, UserIndexSelector, model::IndexValue};

use crate::MetaService;

impl MetaService {
  /// Fetch a [`User`] by [`EmailAddress`].
  #[tracing::instrument(skip(self))]
  pub async fn fetch_user_by_email(
    &self,
    email: EmailAddress,
  ) -> Result<Option<User>, DatabaseError> {
    self
      .user_repo
      .find_by_unique_index(
        UserIndexSelector::Email,
        &IndexValue::new_single(&email),
      )
      .await
  }
}
