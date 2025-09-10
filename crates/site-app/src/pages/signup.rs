use leptos::{ev::SubmitEvent, prelude::*};

use crate::{
  components::{
    form_layout::*, HideableInputField, InputField, InputIcon, LoadingCircle,
  },
  hooks::SignupHook,
};

#[component]
pub fn SignupPage() -> impl IntoView {
  view! {
    <div class="flex-1" />
    <SignupIsland />
    <div class="flex-1" />
  }
}

#[island]
fn SignupIsland() -> impl IntoView {
  let signup_hook = SignupHook::new();

  let name_bindings = signup_hook.name_bindings();
  let email_bindings = signup_hook.email_bindings();
  let password_bindings = signup_hook.password_bindings();
  let confirm_password_bindings = signup_hook.confirm_password_bindings();
  let name_error_hint = MaybeProp::derive(signup_hook.name_error_hint());
  let email_error_hint = MaybeProp::derive(signup_hook.email_error_hint());
  let password_error_hint =
    MaybeProp::derive(signup_hook.password_error_hint());
  let confirm_password_error_hint =
    MaybeProp::derive(signup_hook.confirm_password_error_hint());

  let signup_trigger = signup_hook.action_trigger();
  let submit_action = move |ev: SubmitEvent| {
    ev.prevent_default();
    signup_trigger();
  };
  let show_spinner = signup_hook.show_spinner();

  let _ = signup_hook.create_redirect_effect();

  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-3xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <form on:submit=submit_action class=FORM_CLASS>
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Sign Up"</p>
          <p class="max-w-prose">
            "Thanks so much for trying us out — we can't wait to get you up and \
            running in no time. Prepare for magical iteration cycle times."
          </p>
        </div>
      </GridRowFull>

      <GridRow>
        <GridRowLabel
          title="Your name"
          desc="What do you like to be called?"
        />

        <InputField
          id="name" input_type="text" placeholder="Full Name"
          autofocus=true
          input_signal=name_bindings.0 output_signal=name_bindings.1
          before={InputIcon::User}
          error_hint=name_error_hint
        />
      </GridRow>

      <GridRow>
        <GridRowLabel
          title="Your email"
          desc="[Helpful description goes here]"
        />

        <InputField
          id="email" input_type="text" placeholder="Email Address"
          input_signal=email_bindings.0 output_signal=email_bindings.1
          before={InputIcon::Envelope}
          error_hint=email_error_hint
        />
      </GridRow>

      <GridRow>
        <GridRowLabel
          title="Pick a password"
          desc="[Helpful description goes here]"
        />

        <div class="flex flex-col gap-1">
          <HideableInputField
            id="password" placeholder="Password"
            input_signal=password_bindings.0 output_signal=password_bindings.1
            before={InputIcon::LockClosed}
            error_hint=password_error_hint
          />
          <HideableInputField
            id="confirm_password" placeholder="Confirm Password"
            input_signal=confirm_password_bindings.0 output_signal=confirm_password_bindings.1
            before={InputIcon::LockClosed}
            error_hint=confirm_password_error_hint
          />
        </div>
      </GridRow>

      <GridRow>
        <div />
        <label>
          <input type="submit" class="hidden" />
          <button class="btn btn-primary w-full max-w-80 justify-between">
            <div class="size-4" />
            { signup_hook.button_text() }
            <LoadingCircle {..}
              class="size-4 transition-opacity"
              class=("opacity-0", move || { !show_spinner() })
            />
          </button>
        </label>
      </GridRow>
    </form>
  }
}
