use std::collections::HashMap;

use leptos::{ev::Event, prelude::*};
use models::{EmailAddress, EmailAddressError};

use crate::{
  navigation::{navigate_to, next_url_string_hook},
  reactive_utils::touched_input_bindings,
};

// #[derive(Clone, Copy)]
pub struct LoginHook {
  email_signal:          RwSignal<String>,
  password_signal:       RwSignal<String>,
  submit_touched_signal: RwSignal<bool>,
  next_url_memo:         Signal<String>,
  action:                Action<(), Result<bool, String>>,
}

impl LoginHook {
  pub fn new() -> Self {
    let email_signal = RwSignal::new(String::new());
    let password_signal = RwSignal::new(String::new());
    let submit_touched_signal = RwSignal::new(false);
    let next_url_memo = next_url_string_hook();

    let action = Action::new_local(move |(): &()| {
      // json body for authenticate endpoint
      let body = HashMap::<_, String>::from_iter([
        ("email", email_signal.get()),
        ("password", password_signal.get()),
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

    Self {
      email_signal,
      password_signal,
      submit_touched_signal,
      next_url_memo,
      action,
    }
  }

  pub fn email_bindings(&self) -> (Callback<(), String>, Callback<Event>) {
    touched_input_bindings(self.email_signal)
  }

  pub fn password_bindings(&self) -> (Callback<(), String>, Callback<Event>) {
    touched_input_bindings(self.password_signal)
  }

  pub fn email_error_hint(&self) -> Signal<Option<String>> {
    let (email_signal, submit_touched_signal) =
      (self.email_signal, self.submit_touched_signal);
    Signal::derive(move || {
      let email = email_signal.get();
      if !submit_touched_signal() {
        return None;
      }
      if email.is_empty() {
        return Some("Email address required.".into());
      }
      match EmailAddress::try_new(email) {
        Ok(_) => None,
        Err(EmailAddressError::TooLong) => {
          Some("That email address looks too long.".into())
        }
        Err(EmailAddressError::InvalidEmail) => {
          Some("That email address doesn't look right.".into())
        }
      }
    })
  }

  pub fn password_error_hint(&self) -> Signal<Option<String>> {
    let (password_signal, submit_touched_signal) =
      (self.password_signal, self.submit_touched_signal);
    Signal::derive(move || {
      let password = password_signal.get();
      if !submit_touched_signal() {
        return None;
      }
      if password.is_empty() {
        return Some("Password required.".into());
      }
      None
    })
  }

  pub fn show_spinner(&self) -> Signal<bool> {
    let (pending, value) = (self.action.pending(), self.action.value());
    // show if the action is loading or completed successfully
    Signal::derive(move || pending() || matches!(value.get(), Some(Ok(true))))
  }

  pub fn button_text(&self) -> Signal<&'static str> {
    let (pending, value) = (self.action.pending(), self.action.value());
    Signal::derive(move || match (value.get(), pending()) {
      // if the action is loading at all
      (_, true) => "Loading...",
      // if it's completed successfully
      (Some(Ok(true)), _) => "Redirecting...",
      // any other state
      _ => "Log In",
    })
  }

  pub fn action_trigger(&self) -> Callback<()> {
    let submit_touched_signal = self.submit_touched_signal;
    let email_error_hint = self.email_error_hint();
    let password_error_hint = self.password_error_hint();
    let action = self.action;

    Callback::new(move |()| {
      submit_touched_signal.set(true);

      if email_error_hint().is_some() || password_error_hint().is_some() {
        return;
      }

      action.dispatch_local(());
    })
  }

  pub fn create_redirect_effect(&self) -> Effect<LocalStorage> {
    let (action, next_url_memo) = (self.action, self.next_url_memo);
    Effect::new(move || {
      leptos::logging::log!(
        "redirect effect set to navigate to {:?}",
        next_url_memo()
      );
      if action.value().get() == Some(Ok(true)) {
        navigate_to(&next_url_memo());
      }
    })
  }
}
