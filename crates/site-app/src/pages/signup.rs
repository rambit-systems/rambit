use leptos::{ev::SubmitEvent, prelude::*};

use crate::{
  components::{
    EmailInputField, LoadingCircle, NameInputField, PasswordInputField,
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

  let signup_trigger = signup_hook.action_trigger();
  let submit_action = move |ev: SubmitEvent| {
    ev.prevent_default();
    signup_trigger();
  };
  let show_spinner = signup_hook.show_spinner();

  let _ = signup_hook.create_redirect_effect();

  view! {
    <form
      on:submit=submit_action
      class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8"
    >
      <p class="title">"Sign Up"</p>

      <p class="max-w-prose">
        "Thanks so much for trying us out — we can't wait to get you up and \
        running in no time. Prepare for magical iteration cycle times."
      </p>

      <div class="flex flex-col gap-4">
        <NameInputField
          autofocus=true
          input_signal=name_bindings.0 output_signal=name_bindings.1
          error_hint={MaybeProp::derive(signup_hook.name_error_hint())}
          warn_hint={MaybeProp::from(None::<String>)}
        />
        <EmailInputField
          input_signal=email_bindings.0 output_signal=email_bindings.1
          error_hint={MaybeProp::derive(signup_hook.email_error_hint())}
          warn_hint={MaybeProp::from(None::<String>)}
        />
        <PasswordInputField
          input_signal=password_bindings.0 output_signal=password_bindings.1
          error_hint={MaybeProp::derive(signup_hook.password_error_hint())}
          warn_hint={MaybeProp::from(None::<String>)}
        />
        <PasswordInputField
          id="confirm_password" label_text="Confirm Password"
          input_signal=confirm_password_bindings.0 output_signal=confirm_password_bindings.1
          error_hint={MaybeProp::derive(signup_hook.confirm_password_error_hint())}
          warn_hint={MaybeProp::from(None::<String>)}
        />
      </div>

      <label class="flex flex-row gap-2">
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
    </form>
  }
}
