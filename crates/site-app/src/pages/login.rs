use std::collections::HashMap;

use leptos::{ev::SubmitEvent, prelude::*};
use models::dvf::{EmailAddress, EmailAddressError};

use crate::{
  components::{
    EnvelopeHeroIcon, InputField, LoadingCircle, LockClosedHeroIcon,
  },
  navigation::{navigate_to, next_url_hook},
  reactive_utils::touched_input_bindings,
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
pub fn LoginIsland() -> impl IntoView {
  let email = RwSignal::new(String::new());
  let password = RwSignal::new(String::new());
  let (read_email, write_email) = touched_input_bindings(email);
  let (read_password, write_password) = touched_input_bindings(password);
  let submit_touched = RwSignal::new(false);
  let next_url = next_url_hook();

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

  let password_hint = move || {
    let password = password.get();
    if password.is_empty() {
      return Some("Password required.");
    }
    None
  };

  let action = Action::new_local(move |(): &()| {
    let body = HashMap::<_, String>::from_iter([
      ("email", email.get()),
      ("password", password.get()),
    ]);
    async move {
      let resp = gloo_net::http::Request::post("/api/v1/authenticate")
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

  let submit_action = move |ev: SubmitEvent| {
    ev.prevent_default();

    submit_touched.set(true);

    if email_hint().is_some() || password_hint().is_some() {
      return;
    }

    action.dispatch_local(());
  };

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
      <p class="title">"Login"</p>
      <div class="flex flex-col gap-4">
        <InputField
          id="email" label_text="Email Address"
          input_type="email" placeholder="" autofocus=true
          before={ Box::new(|| view!{ <EnvelopeHeroIcon {..} class="size-6" /> }.into_any()) }
          input_signal=read_email output_signal=write_email
          error_hint={MaybeProp::derive(move || submit_touched().then_some(email_hint()).flatten())}
        />
        <InputField
          id="password" label_text="Password"
          input_type="password" placeholder=""
          before={ Box::new(|| view!{ <LockClosedHeroIcon {..} class="size-6" /> }.into_any()) }
          input_signal=read_password output_signal=write_password
          error_hint={MaybeProp::derive(move || submit_touched().then_some(password_hint()).flatten())}
        />
      </div>

      <label class="flex flex-row gap-2">
        <input type="submit" class="hidden" />
        <button class="btn btn-primary">
          "Log in"
          { move || loading().then_some(view! {
            <LoadingCircle {..} class="size-4" />
          })}
        </button>
      </label>
    </form>
  }
}
