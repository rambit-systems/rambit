use db::{FetchModelByIndexError, kv::LaxSlug};
use models::{User, UserUniqueIndexSelector, dvf::EmailAddress};

use crate::MetaService;

impl MetaService {
  /// Fetch a [`User`] by [`EmailAddress`].
  pub async fn fetch_user_by_email(
    &self,
    email: EmailAddress,
  ) -> Result<Option<User>, FetchModelByIndexError> {
    self
      .user_repo
      .fetch_model_by_unique_index(
        UserUniqueIndexSelector::Email,
        LaxSlug::new(email.as_ref()).into(),
      )
      .await
  }
}
