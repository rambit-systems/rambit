use leptos::prelude::*;

#[component]
pub fn InputField(
  id: &'static str,
  label_text: &'static str,
  input_type: &'static str,
  placeholder: &'static str,
  #[prop(optional)] before: Option<Children>,
  #[prop(optional)] after: Option<Children>,
) -> impl IntoView {
  const OUTER_WRAPPER_CLASS: &str = "flex flex-col gap-1";
  const LABEL_CLASS: &str = "text-base-11";
  const INPUT_WRAPPER_CLASS: &str = "input-field max-w-80";
  const INPUT_CLASS: &str = "w-full py-2 focus-visible:outline-none";

  view! {
    <div class=OUTER_WRAPPER_CLASS>
      <label class=LABEL_CLASS for=id>{ label_text }</label>
      <div class=INPUT_WRAPPER_CLASS>
        { before.map(|b| b()) }
        <input
          class=INPUT_CLASS type=input_type
          placeholder=placeholder id=id
        />
        { after.map(|a| a()) }
      </div>
    </div>
  }
}
