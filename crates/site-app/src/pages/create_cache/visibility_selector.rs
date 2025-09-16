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

  const OUTER_CLASS: &str = "flex-1 flex flex-col gap-2 px-4 py-3 max-w-80 \
                             hover:elevation-lv1 transition rounded border-2 \
                             border-base-7 hover:border-base-8 \
                             bg-gradient-to-tr to-transparent to-50%";
  const OUTER_ACTIVE_CLASS: &str =
    "border-product-7 hover:border-product-8 from-product-3";
  const OUTER_INACTIVE_CLASS: &str = "from-transparent";
  const TITLE_CLASS: &str = "text-base-12 text-lg font-semibold leading-none";
  const DESCRIPTION_CLASS: &str = "text-sm leading-[1.1]";

  let outer_private_class = move || {
    format!(
      "{OUTER_CLASS} {}",
      if is_private() {
        OUTER_ACTIVE_CLASS
      } else {
        OUTER_INACTIVE_CLASS
      }
    )
  };
  let outer_public_class = move || {
    format!(
      "{OUTER_CLASS} {}",
      if is_public() {
        OUTER_ACTIVE_CLASS
      } else {
        OUTER_INACTIVE_CLASS
      }
    )
  };

  view! {
    <div class="flex flex-col gap-4">
      <div
        class=outer_private_class
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
        class=outer_public_class
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
