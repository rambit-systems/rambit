use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, AuthUser, Org};

fn authorize_by_org(org: RecordId<Org>) -> Result<(), ServerFnError> {
  let auth_user: Option<AuthUser> = use_context();
  let cleared_orgs = auth_user
    .map(|au| au.iter_orgs().collect::<Vec<_>>())
    .unwrap_or_default();
  if !cleared_orgs.contains(&org) {
    return Err(ServerFnError::new("Unauthorized"));
  }
  Ok(())
}

pub fn org(id: RecordId<Org>) -> Resource<Result<Option<Org>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_org, move || id)
}

#[server]
async fn fetch_org(id: RecordId<Org>) -> Result<Option<Org>, ServerFnError> {
  use prime_domain::{db::FetchModelError, PrimeDomainService};

  authorize_by_org(id)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service
    .fetch_org_by_id(id)
    .await
    .map_err(|e| match e {
      FetchModelError::Serde(e) => {
        ServerFnError::new(format!("serialization error: {e}"))
      }
      FetchModelError::RetryableTransaction(e) => {
        ServerFnError::new(format!("transaction error: {e}"))
      }
      FetchModelError::Db(e) => {
        ServerFnError::new(format!("unknown db error: {e}"))
      }
    })
}
