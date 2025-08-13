use std::collections::HashMap;

use leptos::{ev::SubmitEvent, prelude::*};
use models::dvf::{EmailAddress, EmailAddressError, HumanName, HumanNameError};

use crate::{
  components::{
    EmailInputField, LoadingCircle, NameInputField, PasswordInputField,
  },
  navigation::{navigate_to, next_url_hook},
  reactive_utils::touched_input_bindings,
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
  let name = RwSignal::new(String::new());
  let email = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());
  let confirm_password = RwSignal::new(String::new());
  let (read_name, write_name) = touched_input_bindings(name);
  let (read_email, write_email) = touched_input_bindings(email);
  let (read_password, write_password) = touched_input_bindings(password);
  let (read_confirm_password, write_confirm_password) =
    touched_input_bindings(confirm_password);
  let submit_touched = RwSignal::new(false);
  let next_url = next_url_hook();

  // error text for name field
  let name_hint = move || {
    let name = name.get();
    if name.is_empty() {
      return Some("Your name is required.");
    }
    match HumanName::try_new(name) {
      Ok(_) => None,
      Err(HumanNameError::LenCharMaxViolated) => {
        Some("The name you entered is too long.")
      }
      Err(HumanNameError::NotEmptyViolated) => Some("Your name is required."),
    }
  };

  // error text for email field
  let email_hint = move || {
    let email = email.get();
    if email.is_empty() {
      return Some("Email address required.");
    }
    match EmailAddress::try_new(email) {
      Ok(_) => None,
      Err(EmailAddressError::LenCharMaxViolated) => {
        Some("That email address looks too long.")
      }
      Err(EmailAddressError::PredicateViolated) => {
        Some("That email address doesn't look right.")
      }
    }
  };

  // error text for password field
  let password_hint = move || {
    let password = password.get();
    if password.is_empty() {
      return Some("Password required.");
    }
    None
  };

  // error text for confirm password field
  let confirm_password_hint = move || {
    let password = password.get();
    let confirm_password = confirm_password.get();
    if confirm_password != password {
      return Some("Passwords don't match.");
    }
    None
  };

  // action to perform login
  let action = Action::new_local(move |(): &()| {
    // json body for authenticate endpoint
    let body = HashMap::<_, String>::from_iter([
      ("name", name.get()),
      ("email", email.get()),
      ("password", password.get()),
    ]);
    async move {
      let resp = gloo_net::http::Request::post("/api/v1/signup")
        .json(&body)
        .expect("failed to build json authenticate payload")
        .send()
        .await
        .map_err(|e| format!("request error: {e}"))?;

      match resp.status() {
        200 => Ok(true),
        401 => Ok(false),
        400 => Err(format!("response error: {}", resp.text().await.unwrap())),
        s => Err(format!("status error: got unknown status {s}")),
      }
    }
  });

  let loading = action.pending();

  // submit callback
  let submit_action = move |ev: SubmitEvent| {
    ev.prevent_default();

    submit_touched.set(true);

    if name_hint().is_some()
      || email_hint().is_some()
      || password_hint().is_some()
      || confirm_password_hint().is_some()
    {
      return;
    }

    action.dispatch_local(());
  };

  // redirect on successful login
  Effect::new(move || {
    if action.value().get() == Some(Ok(true)) {
      navigate_to(&next_url());
    }
  });

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
          input_signal=read_name output_signal=write_name
          error_hint={MaybeProp::derive(move || submit_touched().then_some(name_hint()).flatten())}
        />
        <EmailInputField
          input_signal=read_email output_signal=write_email
          error_hint={MaybeProp::derive(move || submit_touched().then_some(email_hint()).flatten())}
        />
        <PasswordInputField
          input_signal=read_password output_signal=write_password
          error_hint={MaybeProp::derive(move || submit_touched().then_some(password_hint()).flatten())}
        />
        <PasswordInputField
          id="confirm_password" label_text="Confirm Password"
          input_signal=read_confirm_password output_signal=write_confirm_password
          error_hint={MaybeProp::derive(move || submit_touched().then_some(confirm_password_hint()).flatten())}
        />
      </div>

      <label class="flex flex-row gap-2">
        <input type="submit" class="hidden" />
        <button class="btn btn-primary w-full max-w-80">
          <div class="flex-1" />
          <div class="flex-1 flex flex-row justify-center items-center">
            "Log in"
          </div>
          <div class="flex-1 flex flex-row justify-end items-center">
            <LoadingCircle {..}
              class="size-4 transition-opacity"
              class=("opacity-0", move || { !loading() })
            />
          </div>
        </button>
      </label>
    </form>
  }
}
