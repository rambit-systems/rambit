use leptos::prelude::*;
use models::dvf::Visibility;

use crate::hooks::OrgHook;

#[component]
pub(super) fn VisibilitySelector(
  signal: RwSignal<Visibility>,
) -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let org_descriptor = org_hook.descriptor();

  let is_private = move || matches!(signal(), Visibility::Private);
  let is_public = move || matches!(signal(), Visibility::Public);
  let set_private = move |_| signal.set(Visibility::Private);
  let set_public = move |_| signal.set(Visibility::Public);

  const OPTION_CLASS: &str = "flex-1 flex flex-col gap-2 px-4 py-3 \
                              hover:elevation-lv1 transition-shadow \
                              transition-colors rounded border-2 \
                              border-base-7 hover:border-base-8";
  const TITLE_CLASS: &str = "text-lg font-semibold leading-none";
  const DESCRIPTION_CLASS: &str = "text-sm leading-[1.1]";

  view! {
    <div class="flex flex-row gap-4">
      <div
        class=OPTION_CLASS
        class=("border-product-7", is_private)
        class=("hover:border-product-8", is_private)
        on:click=set_private
      >
        <div class="flex flex-row justify-between items-center">
          <p class=TITLE_CLASS>"Private"</p>
          <div class="size-5 bg-base-2 border border-base-6 rounded-full flex flex-row justify-center items-center">
            <div
              class="size-3 bg-product-9 rounded-full transition-opacity"
              class=("opacity-0", move || !is_private())
            />
          </div>
        </div>
        <p class=DESCRIPTION_CLASS>
          "Your entries are only available to users in your organization: "
          <span class="text-base-12">
            <Transition fallback=|| "loading">{ move || Suspend::new(org_descriptor) }</Transition>
          </span>
          "."
        </p>
      </div>

      <div
        class=OPTION_CLASS
        class=("border-product-7", is_public)
        class=("hover:border-product-8", is_public)
        on:click=set_public
      >
        <div class="flex flex-row justify-between items-center">
          <p class=TITLE_CLASS>"Public"</p>
          <div class="size-5 bg-base-2 border border-base-6 rounded-full flex flex-row justify-center items-center">
            <div
              class="size-3 bg-product-9 rounded-full transition-opacity"
              class=("opacity-0", move || !is_public())
            />
          </div>
        </div>
        <p class=DESCRIPTION_CLASS>"Your entries are available for everyone to use."</p>
      </div>
    </div>
  }
}
