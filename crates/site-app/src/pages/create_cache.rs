mod visibility_selector;

use leptos::prelude::*;

use self::visibility_selector::VisibilitySelector;
use crate::{
  components::{form_layout::*, InputField, InputIcon, LoadingCircle},
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

#[component]
pub fn CreateCachePage() -> impl IntoView {
  view! {
    <div class="flex-1" />
    <CreateCacheIsland />
    <div class="flex-1" />
  }
}

#[island]
pub fn CreateCacheIsland() -> impl IntoView {
  let hook = CreateCacheHook::new();

  let (name_bindings, name_error_hint, name_warn_hint, name_after_icon) = (
    hook.name_bindings(),
    hook.name_error_hint(),
    hook.name_warn_hint(),
    hook.name_after_icon(),
  );
  let visibility_signal = hook.visibility_signal();

  let action_trigger = hook.action_trigger();
  let submit_action = move |_| {
    action_trigger.run(());
  };

  let show_spinner = hook.show_spinner();

  let _ = hook.create_redirect_effect();

  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-3xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <div class=FORM_CLASS>
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Create a Cache"</p>
          <p class="max-w-prose whitespace-pre-line">{ CACHE_DESCRIPTION }</p>
        </div>
      </GridRowFull>

      <GridRow>
        <GridRowLabel
          title="Cache name"
          desc="Think of it like a username."
        />

        <InputField
          id="name" label_text="" input_type="text" placeholder="Your name"
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
            "Create Cache"
            <LoadingCircle {..}
              class="size-4 transition-opacity"
              class=("opacity-0", move || { !show_spinner() })
            />
          </button>
        </label>
      </GridRow>
    </div>
  }
}
