use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, Entry};

use crate::resources::entry::entry_query_scope;

#[derive(Clone)]
pub struct EntryHook {
  key:      Callback<(), RecordId<Entry>>,
  resource: Resource<Result<Option<Entry>, ServerFnError>>,
}

impl EntryHook {
  /// Creates a new [`EntryHook`].  
  pub fn new(
    key: impl Fn() -> RecordId<Entry> + Copy + Send + Sync + 'static,
  ) -> Self {
    let client = expect_context::<QueryClient>();
    let resource = client.resource(entry_query_scope(), key);

    Self {
      key: Callback::new(move |_| key()),
      resource,
    }
  }

  pub fn all(&self) -> AsyncDerived<Result<Option<Entry>, ServerFnError>> {
    AsyncDerived::new({
      let resource = self.resource;
      move || async move { resource.await }
    })
  }
}
