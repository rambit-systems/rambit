use leptos::{ev::Event, prelude::*};

#[component]
pub fn InputField(
  id: &'static str,
  label_text: &'static str,
  input_type: &'static str,
  placeholder: &'static str,
  #[prop(optional)] before: Option<Children>,
  #[prop(optional)] after: Option<Children>,
  input_signal: impl Fn() -> String + Send + 'static,
  output_signal: impl Fn(Event) + Send + 'static,
  #[prop(into)] error_hint: MaybeProp<&'static str>,
) -> impl IntoView {
  const OUTER_WRAPPER_CLASS: &str = "flex flex-col gap-1";
  const LABEL_CLASS: &str = "text-base-11";
  const INPUT_WRAPPER_CLASS: &str = "input-field max-w-80";
  const INPUT_CLASS: &str = "w-full py-2 focus-visible:outline-none";
  const ERROR_HINT_CLASS: &str = "text-critical-11 text-sm";

  view! {
    <div class=OUTER_WRAPPER_CLASS>
      <label class=LABEL_CLASS for=id>{ label_text }</label>
      <div class=INPUT_WRAPPER_CLASS>
        { before.map(|b| b()) }
        <input
          class=INPUT_CLASS type=input_type
          placeholder=placeholder id=id
          on:input={move |ev| output_signal(ev)} prop:value={move || input_signal()}
        />
        { after.map(|a| a()) }
      </div>
      { move || error_hint().map(|e| view! {
        <p class=ERROR_HINT_CLASS>{ e }</p>
      })}
    </div>
  }
}
