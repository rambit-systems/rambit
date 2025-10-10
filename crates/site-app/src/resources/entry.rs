use leptos::prelude::*;
use leptos_fetch::QueryScope;
use models::{dvf::RecordId, model::Model, Entry, Org};

#[cfg(feature = "ssr")]
use crate::resources::authorize_for_org;

pub fn entry_query_scope(
) -> QueryScope<RecordId<Entry>, Result<Option<Entry>, ServerFnError>> {
  QueryScope::new(fetch_entry).with_invalidation_link(move |e| {
    [Entry::TABLE_NAME.to_string(), e.to_string()]
  })
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_entry(
  id: RecordId<Entry>,
) -> Result<Option<Entry>, ServerFnError> {
  use domain::DomainService;

  let domain_service: DomainService = expect_context();

  let entry =
    domain_service
      .meta()
      .fetch_entry_by_id(id)
      .await
      .map_err(|e| {
        tracing::error!("failed to fetch entry: {e}");
        ServerFnError::new("internal error")
      })?;

  if let Some(entry) = &entry {
    authorize_for_org(entry.org)?;
  }

  Ok(entry)
}

pub fn entries_in_org_query_scope(
) -> QueryScope<RecordId<Org>, Result<Vec<Entry>, ServerFnError>> {
  QueryScope::new(fetch_entries_in_org)
    .with_invalidation_link(move |_| [Entry::TABLE_NAME.to_string()])
}

#[server(prefix = "/api/sfn")]
pub async fn fetch_entries_in_org(
  org: RecordId<Org>,
) -> Result<Vec<Entry>, ServerFnError> {
  use domain::DomainService;

  authorize_for_org(org)?;

  let domain_service: DomainService = expect_context();

  let ids = domain_service
    .meta()
    .fetch_entries_by_org(org)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch entries by org: {e}");
      ServerFnError::new("internal error")
    })?;
  let mut models = Vec::with_capacity(ids.len());

  for id in ids {
    models.push(
      domain_service
        .meta()
        .fetch_entry_by_id(id)
        .await
        .map_err(|e| {
          tracing::error!("failed to fetch entry by id: {e}");
          ServerFnError::new("internal error")
        })?
        .ok_or_else(|| {
          tracing::error!("could not find entry just found by org index: {id}");
          ServerFnError::new("internal error")
        })?,
    );
  }

  Ok(models)
}
