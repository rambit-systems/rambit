use leptos::prelude::*;
use leptos_fetch::QueryScope;
use models::{Org, OrgSubscriptionReport, RecordId};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn org_subscription_report_query_scope(
) -> QueryScope<RecordId<Org>, Result<OrgSubscriptionReport, ServerFnError>> {
  QueryScope::new(build_org_subscription_report)
}

#[server(prefix = "/api/sfn")]
pub async fn build_org_subscription_report(
  org_id: RecordId<Org>,
) -> Result<OrgSubscriptionReport, ServerFnError> {
  use domain::DomainService;

  let domain_service: DomainService = expect_context();

  authorize_for_org(org_id)?;

  let report = domain_service
    .org_subscription_report(org_id)
    .await
    .map_err(|e| {
      tracing::error!("failed to build org subscription report: {e:?}");
      ServerFnError::new("internal error")
    })?;

  Ok(report)
}
