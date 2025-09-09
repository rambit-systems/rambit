use leptos::{ev::Event, prelude::*};

use super::{InputField, InputIcon};
use crate::components::HideableInputField;

#[component]
pub fn NameInputField(
  #[prop(default = "name")] id: &'static str,
  #[prop(default = "Full Name")] label_text: &'static str,
  #[prop(default = "text")] input_type: &'static str,
  #[prop(default = "")] placeholder: &'static str,
  #[prop(into, default =
    InputIcon::User.into()
  )]
  before: MaybeProp<InputIcon>,
  #[prop(optional)] after: MaybeProp<InputIcon>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<String>,
  #[prop(into)] warn_hint: MaybeProp<String>,
) -> impl IntoView {
  view! {
    <InputField
      id=id label_text=label_text
      input_type=input_type placeholder=placeholder autofocus=autofocus
      before=before after=after
      input_signal=input_signal output_signal=output_signal
      error_hint=error_hint warn_hint=warn_hint
    />
  }
}

#[component]
pub fn EmailInputField(
  #[prop(default = "email")] id: &'static str,
  #[prop(default = "Email Address")] label_text: &'static str,
  #[prop(default = "text")] input_type: &'static str,
  #[prop(default = "")] placeholder: &'static str,
  #[prop(into, default =
    InputIcon::Envelope.into()
  )]
  before: MaybeProp<InputIcon>,
  #[prop(into, optional)] after: MaybeProp<InputIcon>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<String>,
  #[prop(into)] warn_hint: MaybeProp<String>,
) -> impl IntoView {
  view! {
    <InputField
      id=id label_text=label_text
      input_type=input_type placeholder=placeholder autofocus=autofocus
      before=before after=after
      input_signal=input_signal output_signal=output_signal
      error_hint=error_hint warn_hint=warn_hint
    />
  }
}

#[component]
pub fn PasswordInputField(
  #[prop(default = "password")] id: &'static str,
  #[prop(default = "Password")] label_text: &'static str,
  #[prop(default = "text")] unhidden_input_type: &'static str,
  #[prop(default = "")] placeholder: &'static str,
  #[prop(into, default =
    InputIcon::LockClosed.into()
  )]
  before: MaybeProp<InputIcon>,
  #[prop(into, optional)] after: MaybeProp<InputIcon>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<String>,
  #[prop(into)] warn_hint: MaybeProp<String>,
) -> impl IntoView {
  view! {
    <HideableInputField
      id=id label_text=label_text unhidden_input_type=unhidden_input_type
      placeholder=placeholder autofocus=autofocus
      before=before
      input_signal=input_signal output_signal=output_signal
      error_hint=error_hint warn_hint=warn_hint
    />
  }
}
