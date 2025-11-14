pub mod cache;
pub mod entry;
pub mod org;
pub mod store;
pub mod subscription;

#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use models::{AuthUser, Org, RecordId};

#[cfg(feature = "ssr")]
pub fn authorize_for_org(
  org: RecordId<Org>,
) -> Result<AuthUser, ServerFnError> {
  match authenticate() {
    Ok(auth_user) if auth_user.belongs_to_org(org) => Ok(auth_user),
    Ok(_) => Err(ServerFnError::new("Unauthorized")),
    Err(e) => Err(e),
  }
}

#[cfg(feature = "ssr")]
pub fn authenticate() -> Result<AuthUser, ServerFnError> {
  use_context::<AuthUser>().ok_or(ServerFnError::new("Unauthorized"))
}
