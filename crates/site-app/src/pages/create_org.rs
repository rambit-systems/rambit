use leptos::prelude::*;

use crate::{
  components::{form_layout::*, InputField, InputIcon, LoadingCircle},
  hooks::CreateOrgHook,
};

const ORG_DESCRIPTION: &str =
  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
   tempor incididunt ut labore et dolore magna aliqua.

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
   aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
   voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

#[component]
pub fn CreateOrgPage() -> impl IntoView {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <div class=FORM_CLASS>
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Create an Organization"</p>
          <p class="max-w-prose whitespace-pre-line">{ ORG_DESCRIPTION }</p>
        </div>
      </GridRowFull>

      <GridRowFull>
        <div class="h-0 border-t-[1.5px] border-base-6 w-full" />
      </GridRowFull>

      <CreateOrgIsland />
    </div>
  }
}

#[island]
pub fn CreateOrgIsland() -> impl IntoView {
  let hook = CreateOrgHook::new();

  let name_bindings = hook.name_bindings();
  let name_error_hint = hook.name_error_hint();
  let name_warn_hint = hook.name_warn_hint();
  let name_after_icon = hook.name_after_icon();

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
        title="Org name"
        desc="Think of it like a username."
      />

      <InputField
        id="name" label_text="" input_type="text" placeholder="Org Name"
        before=InputIcon::BuildingOffice
        after=name_after_icon
        input_signal=name_bindings.0 output_signal=name_bindings.1
        error_hint=name_error_hint warn_hint=name_warn_hint autofocus=true
      />
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
