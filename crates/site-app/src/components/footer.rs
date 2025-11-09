use leptos::prelude::*;

const LICENSE_HREF: &str =
  "https://github.com/rambit-systems/rambit/blob/main/LICENSE.md";
const PORRIDGE_CO_HREF: &str = "https://github.com/porridge-co";

#[component]
pub fn Footer() -> impl IntoView {
  const CLASS: &str = "elevation-flat flex flex-row px-4 gap-4 items-center \
                       h-14 rounded-b-none! mt-8 text-sm";
  const LINK_CLASS: &str = "text-link";

  view! {
    <div class=CLASS>
      <a
        href=LICENSE_HREF class=LINK_CLASS
        target="_blank" rel="noopener noreferrer"
      >"License"</a>

      <div class="flex-1" />

      <p>
        "Made with zeal by "
        <a
          href=PORRIDGE_CO_HREF class=LINK_CLASS
          target="_blank" rel="noopener noreferrer"
        >"Porridge Co."</a>
      </p>
    </div>
  }
}
