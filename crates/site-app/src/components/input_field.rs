mod specialized;

use leptos::{ev::Event, prelude::*};

pub use self::specialized::*;
use crate::components::{
  icons::LockClosedHeroIcon, ArchiveBoxHeroIcon, EnvelopeHeroIcon, UserHeroIcon,
};

#[derive(Clone, Copy)]
pub enum InputFieldIcon {
  ArchiveBox,
  Envelope,
  LockClosed,
  User,
}

impl IntoAny for InputFieldIcon {
  fn into_any(self) -> AnyView {
    match self {
      InputFieldIcon::ArchiveBox => {
        view! { <ArchiveBoxHeroIcon {..} class="size-6" /> }.into_any()
      }
      InputFieldIcon::Envelope => {
        view! { <EnvelopeHeroIcon {..} class="size-6" /> }.into_any()
      }
      InputFieldIcon::LockClosed => {
        view! { <LockClosedHeroIcon {..} class="size-6" /> }.into_any()
      }
      InputFieldIcon::User => {
        view! { <UserHeroIcon {..} class="size-6" /> }.into_any()
      }
    }
  }
}

#[component]
pub fn InputField(
  id: &'static str,
  label_text: &'static str,
  input_type: &'static str,
  placeholder: &'static str,
  #[prop(optional_no_strip)] before: Option<InputFieldIcon>,
  #[prop(optional_no_strip)] after: Option<InputFieldIcon>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<String>,
  #[prop(into)] warn_hint: MaybeProp<String>,
) -> impl IntoView {
  const OUTER_WRAPPER_CLASS: &str = "flex flex-col gap-1";
  const LABEL_CLASS: &str = "text-base-11";
  const INPUT_WRAPPER_CLASS: &str = "input-field max-w-80";
  const INPUT_CLASS: &str = "w-full py-2 focus-visible:outline-none";
  const ERROR_HINT_CLASS: &str = "text-critical-11 text-sm";
  const WARN_HINT_CLASS: &str = "text-warn-11 text-sm";

  view! {
    <div class=OUTER_WRAPPER_CLASS>
      <label class=LABEL_CLASS for=id>{ label_text }</label>
      <div class=INPUT_WRAPPER_CLASS>
        { move || before.map(|i| i.into_any()).unwrap_or(().into_any()) }
        <input
          class=INPUT_CLASS type=input_type autofocus=autofocus
          placeholder=placeholder id=id
          on:input={move |ev| output_signal(ev)} prop:value={move || input_signal()}
        />
        { move || after.map(|i| i.into_any()).unwrap_or(().into_any()) }
      </div>
      { move || error_hint().map(|e| view! {
        <p class=ERROR_HINT_CLASS>{ e }</p>
      })}
      { move || warn_hint().map(|e| view! {
        <p class=WARN_HINT_CLASS>{ e }</p>
      })}
    </div>
  }
}
