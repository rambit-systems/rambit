pub mod cache;
pub mod entry;
pub mod org;
pub mod store;

#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use models::{dvf::RecordId, AuthUser, Org};

#[cfg(feature = "ssr")]
fn authorize_for_org(org: RecordId<Org>) -> Result<AuthUser, ServerFnError> {
  match use_context::<AuthUser>() {
    Some(auth_user) if auth_user.belongs_to_org(org) => Ok(auth_user),
    Some(_) => Err(ServerFnError::new("Unauthorized")),
    None => Err(ServerFnError::new("Unauthenticated")),
  }
}
