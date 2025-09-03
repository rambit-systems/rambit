mod specialized;

use leptos::{ev::Event, prelude::*};

pub use self::specialized::*;

#[component]
pub fn InputField(
  id: &'static str,
  label_text: &'static str,
  input_type: &'static str,
  placeholder: &'static str,
  #[prop(optional_no_strip)] before: Option<AnyView>,
  #[prop(optional_no_strip)] after: Option<AnyView>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(default = false)] autofocus: bool,
  #[prop(into)] error_hint: MaybeProp<&'static str>,
) -> impl IntoView {
  const OUTER_WRAPPER_CLASS: &str = "flex flex-col gap-1 max-w-80";
  const LABEL_CLASS: &str = "text-base-11";
  const INPUT_WRAPPER_CLASS: &str = "input-field";
  const INPUT_CLASS: &str = "w-full py-2 focus-visible:outline-none";
  const ERROR_HINT_CLASS: &str = "text-critical-11 text-sm";

  view! {
    <div class=OUTER_WRAPPER_CLASS>
      <label class=LABEL_CLASS for=id>{ label_text }</label>
      <div class=INPUT_WRAPPER_CLASS>
        { before }
        <input
          class=INPUT_CLASS type=input_type autofocus=autofocus
          placeholder=placeholder id=id
          on:input={move |ev| output_signal(ev)} prop:value={move || input_signal()}
        />
        { after }
      </div>
      { move || error_hint().map(|e| view! {
        <p class=ERROR_HINT_CLASS>{ e }</p>
      })}
    </div>
  }
}
