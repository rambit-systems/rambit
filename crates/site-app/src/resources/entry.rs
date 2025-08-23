use std::time::Duration;

use leptos::prelude::*;
use leptos_fetch::{QueryOptions, QueryScope};
use models::{dvf::RecordId, model::Model, Entry, Org};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn entries_in_org_query_scope(
) -> QueryScope<RecordId<Org>, Result<Vec<Entry>, ServerFnError>> {
  QueryScope::new(fetch_entries_in_org)
    .with_options(
      QueryOptions::new().with_refetch_interval(Duration::from_secs(5)),
    )
    .with_invalidation_link(move |_| [Entry::TABLE_NAME.to_string()])
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_entries_in_org(
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
