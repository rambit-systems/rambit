use std::time::Duration;

use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryOptions, QueryScope};
use models::{dvf::RecordId, Entry, Org};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn entries_in_org(
  key: impl Fn() -> RecordId<Org> + Send + Sync + 'static,
) -> Resource<Result<Vec<Entry>, ServerFnError>> {
  let client = expect_context::<QueryClient>();
  client.resource(entries_in_org_query_scope(), key)
}

pub fn entries_in_org_query_scope(
) -> QueryScope<RecordId<Org>, Result<Vec<Entry>, ServerFnError>> {
  QueryScope::new(fetch_entries_in_org).with_options(
    QueryOptions::new().with_refetch_interval(Duration::from_secs(5)),
  )
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
