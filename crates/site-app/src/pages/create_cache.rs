mod visibility_selector;

use leptos::prelude::*;

use self::visibility_selector::VisibilitySelector;
use crate::{
  components::{InputField, InputIcon, LoadingCircle},
  hooks::CreateCacheHook,
};

const CACHE_DESCRIPTION: &str =
  "A cache is a container and access-control mechanism for entries, and is \
   the primary namespace through which users will consume your entries. It \
   has a publicly-accessible name which must be globally unique (across \
   organizations). The cache's visibility controls whether its entries are \
   accessible outside of your organization.

   Generally cache names are on a first-come-first-served basis, but contact \
   us if you have concerns.";

#[island]
pub fn CreateCachePage() -> impl IntoView {
  let hook = CreateCacheHook::new();

  let name_bindings = hook.name_bindings();

  let action_trigger = hook.action_trigger();
  let submit_action = move |_| {
    action_trigger.run(());
  };

  let show_spinner = hook.show_spinner();

  let _ = hook.create_redirect_effect();

  view! {
    <div class="flex-1" />
    <div class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8">
      <p class="title">"Create a Cache"</p>

      <p class="max-w-prose whitespace-pre-line">{ CACHE_DESCRIPTION }</p>

      <div class="h-0 border-t-[1.5px] border-base-6 w-full" />

      <div class="flex flex-col gap-4">
        <InputField
          id="name" label_text="Cache Name" input_type="text" placeholder=""
          before=InputIcon::ArchiveBox
          after={ hook.name_after_icon() }
          input_signal=name_bindings.0 output_signal=name_bindings.1
          error_hint={ hook.name_error_hint() } warn_hint={ hook.name_warn_hint() } autofocus=true
        />

        <div class="flex flex-col gap-1">
          <p class="text-11-base">"Visibility"</p>
          <VisibilitySelector signal={ hook.visibility_signal() } />
        </div>
      </div>

      <label class="flex flex-row gap-2">
        <input type="submit" class="hidden" />
        <button
          class="btn btn-primary w-full max-w-80 justify-between"
          on:click=submit_action
        >
          <div class="size-4" />
          "Create Cache"
          <LoadingCircle {..}
            class="size-4 transition-opacity"
            class=("opacity-0", move || { !show_spinner() })
          />
        </button>
      </label>
    </div>
    <div class="flex-1" />
  }
}
