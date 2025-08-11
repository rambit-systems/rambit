use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Org};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn org(id: RecordId<Org>) -> Resource<Result<Option<Org>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_org, move || id)
}

#[server(prefix = "/api/sfn")]
async fn fetch_org(id: RecordId<Org>) -> Result<Option<Org>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(id)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service.fetch_org_by_id(id).await.map_err(|e| {
    tracing::error!("failed to fetch org: {e}");
    ServerFnError::new("internal error")
  })
}
