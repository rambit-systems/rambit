use leptos::prelude::*;

use super::VISIBILITY_FIELD_NAME;
use crate::hooks::OrgHook;

#[component]
pub(super) fn VisibilitySelector() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let org_descriptor = org_hook.descriptor();

  view! {
    <fieldset class="flex flex-col gap-4">
      <Option name="Private" input_name=VISIBILITY_FIELD_NAME value="private" default=true>
        "Your entries are only available to users in your organization: "
        <span class="text-base-12">
          <Transition fallback=|| "loading">{ move || Suspend::new(org_descriptor) }</Transition>
        </span>
        "."
      </Option>

      <Option name="Public" input_name=VISIBILITY_FIELD_NAME value="public">
        "Your entries are available for everyone to use."
      </Option>
    </fieldset>
  }
}

#[component]
fn Option(
  name: impl AsRef<str>,
  input_name: impl AsRef<str>,
  value: impl AsRef<str>,
  children: Children,
  #[prop(optional)] default: bool,
) -> impl IntoView {
  const OUTER_CLASS: &str =
    "group flex-1 flex flex-col gap-2 px-4 py-3 max-w-80 hover:elevation-lv1 \
     cursor-pointer transition rounded border-2 border-base-7 \
     hover:border-base-8 bg-gradient-to-tr from-transparent to-transparent \
     to-50% has-checked:border-product-7 has-checked:hover:border-product-8 \
     has-checked:from-product-3";
  const TITLE_CLASS: &str = "text-base-12 text-lg font-semibold leading-none";
  const DESCRIPTION_CLASS: &str = "text-sm leading-[1.1]";
  const BUTTON_BACKGROUND_CLASS: &str = "size-5 bg-base-2 border \
                                         border-base-6 rounded-full flex \
                                         flex-row justify-center items-center";
  const BUTTON_CLASS: &str = "size-3 bg-product-9 rounded-full \
                              transition-opacity opacity-0 \
                              group-has-checked:opacity-100";

  view! {
    <label class=OUTER_CLASS>
      <input
        type="radio"
        name={ input_name.as_ref() }
        value={ value.as_ref() }
        class="hidden"
        required=true
        checked=default
      />
      <div class="flex flex-row justify-between items-center">
        <p class=TITLE_CLASS>{ name.as_ref() }</p>
        <div class=BUTTON_BACKGROUND_CLASS>
          <div class=BUTTON_CLASS />
        </div>
      </div>
      <div class=DESCRIPTION_CLASS>{ children() }</div>
    </label>
  }
}
