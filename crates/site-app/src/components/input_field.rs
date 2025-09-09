mod icon;

use leptos::{
  ev::{Event, MouseEvent},
  prelude::*,
};

pub use self::icon::*;

#[component]
pub fn InputField(
  #[prop(into)] id: Signal<&'static str>,
  #[prop(into, optional_no_strip)] label_text: MaybeProp<&'static str>,
  #[prop(into)] input_type: Signal<&'static str>,
  #[prop(into, optional_no_strip)] placeholder: MaybeProp<&'static str>,

  #[prop(into, optional_no_strip)] before: MaybeProp<InputIcon>,
  #[prop(into, optional_no_strip)] after: MaybeProp<InputIcon>,
  #[prop(into, optional_no_strip)] before_click_callback: Option<
    Callback<MouseEvent>,
  >,
  #[prop(into, optional_no_strip)] after_click_callback: Option<
    Callback<MouseEvent>,
  >,

  #[prop(into)] input_signal: Callback<(), String>,
  #[prop(into)] output_signal: Callback<Event>,

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
          on:input={move |ev| output_signal.run(ev)} prop:value={move || input_signal.run(())}
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

  #[prop(into)] id: Signal<&'static str>,
  #[prop(into, optional_no_strip)] label_text: MaybeProp<&'static str>,
  #[prop(into, default = "text".into())] unhidden_input_type: Signal<
    &'static str,
  >,
  #[prop(into, optional_no_strip)] placeholder: MaybeProp<&'static str>,

  #[prop(into, optional_no_strip)] before: MaybeProp<InputIcon>,
  #[prop(into, optional_no_strip)] before_click_callback: Option<
    Callback<MouseEvent>,
  >,

  #[prop(into)] input_signal: Callback<(), String>,
  #[prop(into)] output_signal: Callback<Event>,

  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<String>,
  #[prop(into)] warn_hint: MaybeProp<String>,
) -> impl IntoView {
  let input_visible = RwSignal::new(!hidden_by_default);
  let input_type = Signal::derive(move || match input_visible() {
    true => unhidden_input_type(),
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
