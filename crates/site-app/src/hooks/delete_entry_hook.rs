use leptos::{prelude::*, server::ServerAction};
use models::{Entry, RecordId};

use crate::{hooks::OrgHook, navigation::navigate_to};

pub struct DeleteEntryHook {
  key:    Callback<(), RecordId<Entry>>,
  action: ServerAction<DeleteEntry>,
}

impl DeleteEntryHook {
  pub fn new(
    key: impl Fn() -> RecordId<Entry> + Copy + Send + Sync + 'static,
  ) -> Self {
    Self {
      key:    Callback::new(move |()| key()),
      action: ServerAction::new(),
    }
  }

  pub fn show_spinner(&self) -> Signal<bool> {
    let (pending, value) = (self.action.pending(), self.action.value());
    // show if the action is loading or completed successfully
    Signal::derive(move || pending() || matches!(value.get(), Some(Ok(_))))
  }

  pub fn button_text(&self) -> Signal<&'static str> {
    let (pending, value) = (self.action.pending(), self.action.value());
    Signal::derive(move || match (value.get(), pending()) {
      // if the action is loading at all
      (_, true) => "Deleting...",
      // if it's completed successfully
      (Some(Ok(_)), _) => "Redirecting...",
      // any other state
      _ => "Delete Entry",
    })
  }

  pub fn action_trigger(&self) -> Callback<()> {
    let (key, action) = (self.key, self.action);
    Callback::new(move |()| {
      action.dispatch(DeleteEntry { id: key.run(()) });
    })
  }

  pub fn create_redirect_effect(&self) -> Effect<LocalStorage> {
    let action = self.action;
    Effect::new(move || {
      if matches!(action.value().get(), Some(Ok(_))) {
        let org_hook = OrgHook::new_requested();
        navigate_to(&org_hook.dashboard_url()());
      }
    })
  }
}

#[server(prefix = "/api/sfn")]
async fn delete_entry(
  id: RecordId<Entry>,
) -> Result<Option<RecordId<Entry>>, ServerFnError> {
  use domain::{db::DatabaseError, DomainService};

  let domain_service = expect_context::<DomainService>();

  let entry =
    domain_service
      .meta()
      .fetch_entry_by_id(id)
      .await
      .map_err(|e| {
        tracing::error!("failed to fetch entry to delete: {e}");
        ServerFnError::new("internal error")
      })?;
  let Some(entry) = entry else {
    return Ok(None);
  };

  crate::resources::authorize_for_org(entry.org)?;

  match domain_service.delete_entry(id).await {
    Ok(entry) => Ok(Some(entry.id)),
    Err(DatabaseError::NotFound(_)) => Ok(None),
    Err(e) => {
      tracing::error!("failed to delete entry: {e}");
      Err(ServerFnError::new("internal error"))
    }
  }
}
