use db::DatabaseError;
use models::{EntityName, RecordId, Store, StoreIndexSelector, User};

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
  DatabaseError(#[from] DatabaseError),
}

impl MetaService {
  /// Find all stores with the given name across all a user's orgs.
  #[tracing::instrument(skip(self))]
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
      let index_value = Store::unique_index_name_by_org(org, &store_name);

      // find the associated store
      let store = self
        .store_repo
        .find_by_unique_index(StoreIndexSelector::NameByOrg, &index_value)
        .await?;

      // if a store with that name exists, push it
      if let Some(store) = store {
        stores.push(store);
      }
    }

    Ok(stores)
  }
}
