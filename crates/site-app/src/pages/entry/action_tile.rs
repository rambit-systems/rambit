use leptos::prelude::*;
use models::{dvf::RecordId, Entry};

use crate::{components::LoadingCircle, hooks::DeleteEntryHook};

#[island]
pub(crate) fn ActionTile(entry_id: RecordId<Entry>) -> impl IntoView {
  let delete_hook = DeleteEntryHook::new(move || entry_id);
  let show_delete_spinner = delete_hook.show_spinner();
  let delete_button_text = delete_hook.button_text();
  let delete_dispatcher = delete_hook.action_trigger();
  let _ = delete_hook.create_redirect_effect();

  view! {
    <div class="md:w-64 p-6 elevation-flat flex flex-col gap-4 align-self-start">
      <p class="subtitle">"Actions"</p>
      <div class="flex flex-col gap-2">
        <button
          class="btn btn-critical justify-between"
          on:click={move |_| delete_dispatcher.run(())}
        >
          <div class="size-4" />
          { delete_button_text }
          <LoadingCircle {..}
            class="size-4 transition-opacity"
            class=("opacity-0", move || { !show_delete_spinner() })
          />
        </button>
      </div>
    </div>
  }
}
