use leptos::{ev::Event, prelude::*};

use super::InputField;
use crate::components::{EnvelopeHeroIcon, LockClosedHeroIcon, UserHeroIcon};

#[component]
pub fn NameInputField(
  #[prop(default = "name")] id: &'static str,
  #[prop(default = "Full Name")] label_text: &'static str,
  #[prop(default = "text")] input_type: &'static str,
  #[prop(default = "")] placeholder: &'static str,
  #[prop(default =
Some(Box::new(|| view!{ <UserHeroIcon {..} class="size-6" /> }.into_any()))
  )]
  before: Option<Children>,
  #[prop(optional)] after: Option<Children>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<&'static str>,
) -> impl IntoView {
  view! {
    <InputField
      id=id label_text=label_text
      input_type=input_type placeholder=placeholder autofocus=autofocus
      before=before after=after
      input_signal=input_signal output_signal=output_signal
      error_hint=error_hint
    />
  }
}

#[component]
pub fn EmailInputField(
  #[prop(default = "email")] id: &'static str,
  #[prop(default = "Email Address")] label_text: &'static str,
  #[prop(default = "text")] input_type: &'static str,
  #[prop(default = "")] placeholder: &'static str,
  #[prop(default =
Some(Box::new(|| view!{ <EnvelopeHeroIcon {..} class="size-6" /> }.into_any()))
  )]
  before: Option<Children>,
  #[prop(optional)] after: Option<Children>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<&'static str>,
) -> impl IntoView {
  view! {
    <InputField
      id=id label_text=label_text
      input_type=input_type placeholder=placeholder autofocus=autofocus
      before=before after=after
      input_signal=input_signal output_signal=output_signal
      error_hint=error_hint
    />
  }
}

#[component]
pub fn PasswordInputField(
  #[prop(default = "password")] id: &'static str,
  #[prop(default = "Password")] label_text: &'static str,
  #[prop(default = "password")] input_type: &'static str,
  #[prop(default = "")] placeholder: &'static str,
  #[prop(default =
Some(Box::new(|| view!{ <LockClosedHeroIcon {..} class="size-6" /> }.into_any()))
  )]
  before: Option<Children>,
  #[prop(optional)] after: Option<Children>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<&'static str>,
) -> impl IntoView {
  view! {
    <InputField
      id=id label_text=label_text
      input_type=input_type placeholder=placeholder autofocus=autofocus
      before=before after=after
      input_signal=input_signal output_signal=output_signal
      error_hint=error_hint
    />
  }
}
