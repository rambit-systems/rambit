use leptos::prelude::*;
use leptos_fetch::QueryScope;
use models::{dvf::RecordId, model::Model, Org, PvOrg};

#[cfg(feature = "ssr")]
use crate::resources::{authenticate, authorize_for_org};

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

pub fn org_name_is_available_query_scope(
) -> QueryScope<String, Result<bool, ServerFnError>> {
  QueryScope::new(check_if_org_name_is_available)
    .with_invalidation_link(move |_| [Org::TABLE_NAME])
}

#[server(prefix = "/api/sfn")]
pub async fn check_if_org_name_is_available(
  name: String,
) -> Result<bool, ServerFnError> {
  use models::dvf::{EntityName, StrictSlug};
  use prime_domain::PrimeDomainService;

  authenticate()?;

  let sanitized_name = EntityName::new(StrictSlug::new(name.clone()));
  if name != sanitized_name.clone().to_string() {
    return Err(ServerFnError::new("name is unsanitized"));
  }

  let prime_domain_service: PrimeDomainService = expect_context();

  let occupied = prime_domain_service
    .fetch_org_by_ident(models::OrgIdent::Named(sanitized_name))
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch org by name: {e}");
      ServerFnError::new("internal error")
    })?
    .is_some();

  Ok(!occupied)
}
