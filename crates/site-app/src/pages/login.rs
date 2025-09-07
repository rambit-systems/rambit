use leptos::{ev::SubmitEvent, prelude::*};

use crate::{
  components::{EmailInputField, LoadingCircle, PasswordInputField},
  hooks::LoginHook,
};

#[component]
pub fn LoginPage() -> impl IntoView {
  view! {
    <div class="flex-1" />
    <LoginIsland />
    <div class="flex-1" />
  }
}

#[island]
fn LoginIsland() -> impl IntoView {
  let login_hook = LoginHook::new();

  let email_bindings = login_hook.email_bindings();
  let password_bindings = login_hook.password_bindings();

  let signup_trigger = login_hook.action_trigger();
  let submit_action = move |ev: SubmitEvent| {
    ev.prevent_default();
    signup_trigger();
  };
  let show_spinner = login_hook.show_spinner();

  let _ = login_hook.create_redirect_effect();

  view! {
    <form
      on:submit=submit_action
      class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8"
    >
      <p class="title">"Login"</p>

      <p class="max-w-prose">
        "Welcome back to the most satisfying part of your CI/CD pipeline."
      </p>

      <div class="flex flex-col gap-4">
        <EmailInputField
          autofocus=true
          input_signal=email_bindings.0 output_signal=email_bindings.1
          error_hint={MaybeProp::derive(login_hook.email_error_hint())}
          warn_hint={MaybeProp::from(None::<String>)}
        />
        <PasswordInputField
          input_signal=password_bindings.0 output_signal=password_bindings.1
          error_hint={MaybeProp::derive(login_hook.password_error_hint())}
          warn_hint={MaybeProp::from(None::<String>)}
        />
      </div>

      <label class="flex flex-row gap-2">
        <input type="submit" class="hidden" />
        <button class="btn btn-primary w-full max-w-80 justify-between">
          <div class="size-4" />
          { login_hook.button_text() }
          <LoadingCircle {..}
            class="size-4 transition-opacity"
            class=("opacity-0", move || { !show_spinner() })
          />
        </button>
      </label>
    </form>
  }
}
