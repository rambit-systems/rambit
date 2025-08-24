use leptos::prelude::*;
use models::{dvf::RecordId, Entry};

#[derive(Clone)]
pub struct EntryHook {
  key: Callback<(), RecordId<Entry>>,
}
