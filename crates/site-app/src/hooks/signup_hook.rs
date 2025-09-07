use std::collections::HashMap;

use leptos::{ev::Event, prelude::*};
use models::dvf::{EmailAddress, EmailAddressError, HumanName, HumanNameError};

use crate::{
  navigation::{navigate_to, next_url_hook},
  reactive_utils::touched_input_bindings,
};

pub struct SignupHook {
  name_signal:             RwSignal<String>,
  email_signal:            RwSignal<String>,
  password_signal:         RwSignal<String>,
  confirm_password_signal: RwSignal<String>,
  submit_touched_signal:   RwSignal<bool>,
  next_url_memo:           Memo<String>,
  action:                  Action<(), Result<bool, String>>,
}

impl SignupHook {
  pub fn new() -> Self {
    let name_signal = RwSignal::new(String::new());
    let email_signal = RwSignal::new(String::new());
    let password_signal = RwSignal::new(String::new());
    let confirm_password_signal = RwSignal::new(String::new());
    let submit_touched_signal = RwSignal::new(false);
    let next_url_memo = next_url_hook();

    let action = Action::new_local(move |(): &()| {
      // json body for authenticate endpoint
      let body = HashMap::<_, String>::from_iter([
        ("name", name_signal.get()),
        ("email", email_signal.get()),
        ("password", password_signal.get()),
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

    Self {
      name_signal,
      email_signal,
      password_signal,
      confirm_password_signal,
      submit_touched_signal,
      next_url_memo,
      action,
    }
  }

  pub fn name_bindings(&self) -> (impl Fn() -> String, impl Fn(Event)) {
    touched_input_bindings(self.name_signal)
  }

  pub fn email_bindings(&self) -> (impl Fn() -> String, impl Fn(Event)) {
    touched_input_bindings(self.email_signal)
  }

  pub fn password_bindings(&self) -> (impl Fn() -> String, impl Fn(Event)) {
    touched_input_bindings(self.password_signal)
  }

  pub fn confirm_password_bindings(
    &self,
  ) -> (impl Fn() -> String, impl Fn(Event)) {
    touched_input_bindings(self.confirm_password_signal)
  }

  pub fn name_error_hint(&self) -> impl Fn() -> Option<String> {
    let (name_signal, submit_touched_signal) =
      (self.name_signal, self.submit_touched_signal);
    move || {
      let name = name_signal.get();
      if !submit_touched_signal() {
        return None;
      }
      if name.is_empty() {
        return Some("Your name is required.".into());
      }
      match HumanName::try_new(name) {
        Ok(_) => None,
        Err(HumanNameError::LenCharMaxViolated) => {
          Some("The name you entered is too long.".into())
        }
        Err(HumanNameError::NotEmptyViolated) => {
          Some("Your name is required.".into())
        }
      }
    }
  }

  pub fn email_error_hint(&self) -> impl Fn() -> Option<String> {
    let (email_signal, submit_touched_signal) =
      (self.email_signal, self.submit_touched_signal);
    move || {
      let email = email_signal.get();
      if !submit_touched_signal() {
        return None;
      }
      if email.is_empty() {
        return Some("Email address required.".into());
      }
      match EmailAddress::try_new(email) {
        Ok(_) => None,
        Err(EmailAddressError::LenCharMaxViolated) => {
          Some("That email address looks too long.".into())
        }
        Err(EmailAddressError::PredicateViolated) => {
          Some("That email address doesn't look right.".into())
        }
      }
    }
  }

  pub fn password_error_hint(&self) -> impl Fn() -> Option<String> {
    let (password_signal, submit_touched_signal) =
      (self.password_signal, self.submit_touched_signal);
    move || {
      let password = password_signal.get();
      if !submit_touched_signal() {
        return None;
      }
      if password.is_empty() {
        return Some("Password required.".into());
      }
      None
    }
  }

  pub fn confirm_password_error_hint(&self) -> impl Fn() -> Option<String> {
    let (password_signal, confirm_password_signal, submit_touched_signal) = (
      self.password_signal,
      self.confirm_password_signal,
      self.submit_touched_signal,
    );
    move || {
      let password = password_signal.get();
      let confirm_password = confirm_password_signal.get();
      if !submit_touched_signal() {
        return None;
      }
      if confirm_password != password {
        return Some("Passwords don't match.".into());
      }
      None
    }
  }

  pub fn show_spinner(&self) -> impl Fn() -> bool {
    let (pending, value) = (self.action.pending(), self.action.value());
    // show if the action is loading or completed successfully
    move || pending() || matches!(value.get(), Some(Ok(true)))
  }

  pub fn button_text(&self) -> impl Fn() -> &'static str {
    let (pending, value) = (self.action.pending(), self.action.value());
    move || match (value.get(), pending()) {
      // if the action is loading at all
      (_, true) => "Loading...",
      // if it's completed successfully
      (Some(Ok(true)), _) => "Redirecting...",
      // any other state
      _ => "Sign Up",
    }
  }

  pub fn action_trigger(&self) -> impl Fn() {
    let submit_touched_signal = self.submit_touched_signal;
    let name_error_hint = self.name_error_hint();
    let email_error_hint = self.email_error_hint();
    let password_error_hint = self.password_error_hint();
    let confirm_password_error_hint = self.confirm_password_error_hint();
    let action = self.action;

    move || {
      submit_touched_signal.set(true);

      if name_error_hint().is_some()
        || email_error_hint().is_some()
        || password_error_hint().is_some()
        || confirm_password_error_hint().is_some()
      {
        return;
      }

      action.dispatch_local(());
    }
  }

  pub fn create_redirect_effect(&self) -> Effect<LocalStorage> {
    let (action, next_url_memo) = (self.action, self.next_url_memo);
    Effect::new(move || {
      if action.value().get() == Some(Ok(true)) {
        navigate_to(&next_url_memo());
      }
    })
  }
}
