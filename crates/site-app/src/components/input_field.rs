mod specialized;

use leptos::{ev::Event, prelude::*};

pub use self::specialized::*;
use crate::components::{
  icons::LockClosedHeroIcon, ArchiveBoxHeroIcon, EnvelopeHeroIcon,
  GlobeAltHeroIcon, KeyHeroIcon, UserHeroIcon,
};

#[derive(Clone, Copy)]
pub enum InputIcon {
  ArchiveBox,
  Envelope,
  GlobeAlt,
  Key,
  LockClosed,
  User,
}

impl IntoAny for InputIcon {
  fn into_any(self) -> AnyView {
    const ICON_CLASS: &str = "size-6";
    match self {
      InputIcon::ArchiveBox => {
        view! { <ArchiveBoxHeroIcon {..} class=ICON_CLASS /> }.into_any()
      }
      InputIcon::Envelope => {
        view! { <EnvelopeHeroIcon {..} class=ICON_CLASS /> }.into_any()
      }
      InputIcon::GlobeAlt => {
        view! { <GlobeAltHeroIcon {..} class=ICON_CLASS /> }.into_any()
      }
      InputIcon::Key => {
        view! { <KeyHeroIcon {..} class=ICON_CLASS /> }.into_any()
      }
      InputIcon::LockClosed => {
        view! { <LockClosedHeroIcon {..} class=ICON_CLASS /> }.into_any()
      }
      InputIcon::User => {
        view! { <UserHeroIcon {..} class=ICON_CLASS /> }.into_any()
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
  #[prop(into, optional_no_strip)] before: MaybeProp<InputIcon>,
  #[prop(into, optional_no_strip)] after: MaybeProp<InputIcon>,
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
  const HINT_WRAPPER_CLASS: &str = "flex flex-col";
  const ERROR_HINT_CLASS: &str = "text-critical-11 text-sm";
  const WARN_HINT_CLASS: &str = "text-warn-11 text-sm";

  view! {
    <label for=id class=OUTER_WRAPPER_CLASS>
      <p class=LABEL_CLASS>{ label_text }</p>
      <div class=INPUT_WRAPPER_CLASS>
        { move || before().map(|i| i.into_any()) }
        <input
          class=INPUT_CLASS type=input_type autofocus=autofocus
          placeholder=placeholder id=id
          on:input={move |ev| output_signal(ev)} prop:value={move || input_signal()}
        />
        { move || after().map(|i| i.into_any()) }
      </div>
      <div class=HINT_WRAPPER_CLASS>
        { move || error_hint().map(|e| view! {
          <p class=ERROR_HINT_CLASS>{ e }</p>
        })}
        { move || warn_hint().map(|e| view! {
          <p class=WARN_HINT_CLASS>{ e }</p>
        })}
      </div>
    </label>
  }
}
