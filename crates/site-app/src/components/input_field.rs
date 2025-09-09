mod specialized;

use leptos::{
  ev::{Event, MouseEvent},
  prelude::*,
};

pub use self::specialized::*;
use crate::components::{
  icons::LockClosedHeroIcon, ArchiveBoxHeroIcon, EnvelopeHeroIcon, EyeHeroIcon,
  EyeSlashHeroIcon, GlobeAltHeroIcon, KeyHeroIcon, UserHeroIcon,
};

#[derive(Clone, Copy)]
pub enum InputIcon {
  ArchiveBox,
  Envelope,
  Eye,
  EyeSlash,
  GlobeAlt,
  Key,
  LockClosed,
  User,
}

macro_rules! icon_match {
  ($self_expr:expr, $click_handler:expr, $icon_class:expr, {
      $($variant:ident => $component:ident),* $(,)?
  }) => {
    match $self_expr {
      $(
        InputIcon::$variant => view! {
          <$component {..} class=$icon_class on:click=$click_handler />
        }.into_any(),
      )*
    }
  };
}

impl InputIcon {
  fn into_any(self, click_handler: Option<Callback<MouseEvent>>) -> AnyView {
    const ICON_CLASS: &str = "size-6";

    let click_handler = move |e| {
      if let Some(h) = click_handler {
        h.run(e)
      }
    };

    icon_match!(self, click_handler, ICON_CLASS, {
        ArchiveBox => ArchiveBoxHeroIcon,
        Envelope => EnvelopeHeroIcon,
        Eye => EyeHeroIcon,
        EyeSlash => EyeSlashHeroIcon,
        GlobeAlt => GlobeAltHeroIcon,
        Key => KeyHeroIcon,
        LockClosed => LockClosedHeroIcon,
        User => UserHeroIcon,
    })
  }
}

#[component]
pub fn InputField(
  #[prop(into)] id: Signal<&'static str>,
  #[prop(into)] label_text: Signal<&'static str>,
  #[prop(into)] input_type: Signal<&'static str>,
  #[prop(into)] placeholder: Signal<&'static str>,
  #[prop(into, optional_no_strip)] before: MaybeProp<InputIcon>,
  #[prop(into, optional_no_strip)] after: MaybeProp<InputIcon>,
  #[prop(into, optional_no_strip)] before_click_callback: Option<
    Callback<MouseEvent>,
  >,
  #[prop(into, optional_no_strip)] after_click_callback: Option<
    Callback<MouseEvent>,
  >,
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
        { move || before().map(|i| i.into_any(before_click_callback)) }
        <input
          class=INPUT_CLASS type=input_type autofocus=autofocus
          placeholder=placeholder id=id
          on:input={move |ev| output_signal(ev)} prop:value={move || input_signal()}
        />
        { move || after().map(|i| i.into_any(after_click_callback)) }
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

#[component]
pub fn HideableInputField(
  #[prop(default = true)] hidden_by_default: bool,
  id: &'static str,
  label_text: &'static str,
  unhidden_input_type: &'static str,
  placeholder: &'static str,
  #[prop(into, optional_no_strip)] before: MaybeProp<InputIcon>,
  #[prop(into, optional_no_strip)] before_click_callback: Option<
    Callback<MouseEvent>,
  >,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<String>,
  #[prop(into)] warn_hint: MaybeProp<String>,
) -> impl IntoView {
  let input_visible = RwSignal::new(!hidden_by_default);
  let input_type = Signal::derive(move || match input_visible() {
    true => unhidden_input_type,
    false => "password",
  });
  let after = Signal::derive(move || match input_visible() {
    true => InputIcon::Eye,
    false => InputIcon::EyeSlash,
  });
  let after_click_callback = Callback::new(move |_| {
    input_visible.update(move |v| {
      *v = !*v;
    })
  });

  view! {
    <InputField
      id=id label_text=label_text input_type=input_type placeholder=placeholder
      before=before after=after
      before_click_callback=before_click_callback after_click_callback=after_click_callback
      input_signal=input_signal output_signal=output_signal
      autofocus=autofocus
      error_hint=error_hint warn_hint=warn_hint
    />
  }
}
