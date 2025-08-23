use leptos::prelude::*;
use leptos_fetch::QueryScope;
use models::{dvf::RecordId, model::Model, Org, PvOrg};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn org_query_scope(
) -> QueryScope<RecordId<Org>, Result<Option<PvOrg>, ServerFnError>> {
  QueryScope::new(fetch_org).with_invalidation_link(move |o| {
    [Org::TABLE_NAME.to_string(), o.to_string()]
  })
}

#[server(prefix = "/api/sfn")]
async fn fetch_org(id: RecordId<Org>) -> Result<Option<PvOrg>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  authorize_for_org(id)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  prime_domain_service
    .fetch_org_by_id(id)
    .await
    .map(|o| o.map(PvOrg::from))
    .map_err(|e| {
      tracing::error!("failed to fetch org: {e}");
      ServerFnError::new("internal error")
    })
}
