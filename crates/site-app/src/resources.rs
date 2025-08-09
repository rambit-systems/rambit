use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Org};

pub fn org(id: RecordId<Org>) -> Resource<Result<Option<Org>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_org, move || id)
}

#[server]
async fn fetch_org(id: RecordId<Org>) -> Result<Option<Org>, ServerFnError> {
  use prime_domain::{db::FetchModelError, PrimeDomainService};
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
