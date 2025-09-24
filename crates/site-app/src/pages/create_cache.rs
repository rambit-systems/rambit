mod visibility_selector;

use leptos::prelude::*;

use self::visibility_selector::VisibilitySelector;
use crate::{
  components::{form_layout::*, InputField, InputIcon, LoadingCircle},
  hooks::CreateCacheHook,
};

const CACHE_DESCRIPTION: &str =
  "A cache is a container and access-control mechanism for entries, and is \
   the primary namespace through which users will consume your entries.

   A cache's name must be globally unique (across organizations), even if the \
   cache is set to private. The visibility of the cache controls whether its \
   entries are accessible outside of your organization.

   Generally cache names are on a first-come-first-served basis, but please \
   contact us if you have concerns.";

#[component]
pub fn CreateCachePage() -> impl IntoView {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <div class="flex-1" />
    <div class=FORM_CLASS>
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Create a Cache"</p>
          <p class="max-w-prose whitespace-pre-line">{ CACHE_DESCRIPTION }</p>
        </div>
      </GridRowFull>

      <GridRowFull>
        <div class="h-0 border-t-[1.5px] border-base-6 w-full" />
      </GridRowFull>

      <CreateCacheIsland />
    </div>
    <div class="flex-1" />
  }
}

#[island]
pub fn CreateCacheIsland() -> impl IntoView {
  let hook = CreateCacheHook::new();

  let name_bindings = hook.name_bindings();
  let name_error_hint = hook.name_error_hint();
  let name_warn_hint = hook.name_warn_hint();
  let name_after_icon = hook.name_after_icon();

  let visibility_signal = hook.visibility_signal();

  let action_trigger = hook.action_trigger();
  let submit_action = move |_| {
    action_trigger.run(());
  };

  let button_text = hook.button_text();
  let show_spinner = hook.show_spinner();

  let _ = hook.create_redirect_effect();

  view! {
    <GridRow>
      <GridRowLabel
        title="Cache name"
        desc="Think of it like a username."
      />

      <InputField
        id="name" label_text="" input_type="text" placeholder="Cache Name"
        before=InputIcon::ArchiveBox
        after=name_after_icon
        input_signal=name_bindings.0 output_signal=name_bindings.1
        error_hint=name_error_hint warn_hint=name_warn_hint autofocus=true
      />
    </GridRow>

    <GridRow>
      <GridRowLabel
        title="Visibility"
        desc="For the public good or just your team?"
      />

      <VisibilitySelector signal=visibility_signal />
    </GridRow>

    <GridRow>
      <div />
      <label>
        <input type="submit" class="hidden" />
        <button
          class="btn btn-primary w-full max-w-80 justify-between"
          on:click=submit_action
        >
          <div class="size-4" />
          { button_text }
          <LoadingCircle {..}
            class="size-4 transition-opacity"
            class=("opacity-0", move || { !show_spinner() })
          />
        </button>
      </label>
    </GridRow>
  }
}
