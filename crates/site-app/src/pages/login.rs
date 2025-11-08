use leptos::{ev::SubmitEvent, prelude::*};

use crate::{
  components::{HideableInputField, InputField, InputIcon, LoadingCircle},
  hooks::LoginHook,
};

#[component]
pub fn LoginPage() -> impl IntoView {
  view! {
    <LoginIsland />
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
    signup_trigger.run(());
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
        <InputField
          id="email" label_text="Email Address" input_type="text"
          autofocus=true
          input_signal=email_bindings.0 output_signal=email_bindings.1
          before={InputIcon::Envelope}
          error_hint={MaybeProp::derive(login_hook.email_error_hint())}
          warn_hint={MaybeProp::from(None::<String>)}
        />
        <HideableInputField
          id="password" label_text="Password"
          input_signal=password_bindings.0 output_signal=password_bindings.1
          before={InputIcon::LockClosed}
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
