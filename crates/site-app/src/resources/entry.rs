use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Entry, Org};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn entries_in_org(
  org: RecordId<Org>,
) -> Resource<Result<Vec<Entry>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(fetch_entries_in_org, move || org)
}

#[server(prefix = "/api/sfn")]
async fn fetch_entries_in_org(
  org: RecordId<Org>,
) -> Result<Vec<Entry>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(org)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service
    .fetch_entries_by_org(org)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch entries by org: {e}");
      ServerFnError::new("internal error")
    })
}
