pub mod cache;
pub mod entry;
pub mod org;
pub mod store;

#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use models::{dvf::RecordId, Org};

#[cfg(feature = "ssr")]
fn authorize_for_org(org: RecordId<Org>) -> Result<(), ServerFnError> {
  use models::AuthUser;

  let auth_user: Option<AuthUser> = use_context();
  let cleared_orgs = auth_user
    .map(|au| au.iter_orgs().collect::<Vec<_>>())
    .unwrap_or_default();
  if !cleared_orgs.contains(&org) {
    return Err(ServerFnError::new("Unauthorized"));
  }
  Ok(())
}
