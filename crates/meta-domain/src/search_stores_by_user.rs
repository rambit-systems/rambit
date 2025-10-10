use db::{FetchModelByIndexError, FetchModelError, kv::LaxSlug};
use models::{
  Store, StoreUniqueIndexSelector, User,
  dvf::{EntityName, RecordId},
};

use super::MetaService;

/// Failures that can occur when running
/// [`search_stores_by_name_and_user`](MetaService::search_stores_by_name_and_user).
#[derive(Debug, thiserror::Error)]
pub enum SearchByUserError {
  /// Indicates that the user does not exist.
  #[error("Failed to find user: {0}")]
  MissingUser(RecordId<User>),
  /// Indicates that a database error occurred.
  #[error("Failed to fetch users by index")]
  FetchError(#[from] FetchModelError),
  /// Indicates that a database error occurred.
  #[error("Failed to fetch users by index")]
  FetchByIndexError(#[from] FetchModelByIndexError),
}

impl MetaService {
  /// Find all stores with the given name across all a user's orgs.
  pub async fn search_stores_by_name_and_user(
    &self,
    user_id: RecordId<User>,
    store_name: EntityName,
  ) -> Result<Vec<Store>, SearchByUserError> {
    // fetch the user
    let user = self
      .fetch_user_by_id(user_id)
      .await?
      .ok_or(SearchByUserError::MissingUser(user_id))?;

    // the user's orgs
    let user_orgs = user.iter_orgs();

    // find the store with the given name in each org
    let mut stores = Vec::new();
    for org in user_orgs {
      // calculate the `NameByOrg` index
      let index_value = LaxSlug::new(format!("{org}-{store_name}"));
      // find the associated store
      let store = self
        .store_repo
        .fetch_model_by_unique_index(
          StoreUniqueIndexSelector::NameByOrg,
          index_value.into(),
        )
        .await?;

      // if a store with that name exists, push it
      if let Some(store) = store {
        stores.push(store);
      }
    }

    Ok(stores)
  }
}
