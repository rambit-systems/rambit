use leptos::prelude::*;

use crate::components::form_layout::*;

const ORG_DESCRIPTION: &str =
  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
   tempor incididunt ut labore et dolore magna aliqua.

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
   aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
   voluptate velit esse cillum dolore eu fugiat nulla pariatur.";

#[component]
pub fn CreateOrgPage() -> impl IntoView {
  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <div class="flex-1" />
    <div class=FORM_CLASS>
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Create an Organization"</p>
          <p class="max-w-prose whitespace-pre-line">{ ORG_DESCRIPTION }</p>
        </div>
      </GridRowFull>

      <GridRowFull>
        <div class="h-0 border-t-[1.5px] border-base-6 w-full" />
      </GridRowFull>

      // <CreateCacheIsland />
    </div>
    <div class="flex-1" />
  }
}
