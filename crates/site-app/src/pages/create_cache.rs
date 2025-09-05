use leptos::prelude::*;

use crate::{
  components::{ArchiveBoxHeroIcon, InputField, InputFieldIcon},
  reactive_utils::touched_input_bindings,
};

const CACHE_DESCRIPTION: &str =
  "A cache is a container and access-control mechanism for entries, and is \
   the primary namespace through which users will consume your entries. It \
   has a publicly-accessible name which must be globally unique (across \
   organizations). The cache's visibility controls whether its entries are \
   accessible outside of your organization.

   Generally cache names are on a first-come-first-served basis, but contact \
   us if you have concerns.";

#[island]
pub fn CreateCachePage() -> impl IntoView {
  let name = RwSignal::new(String::new());
  let (read_name, write_name) = touched_input_bindings(name);

  // error text for name field
  // let name_hint = move || {
  //   let name = name.get();
  //   if name.is_empty() {
  //     return Some("Your name is required.");
  //   }
  //   EntityName::new(StrictSlug::new(name)) {
  //     Ok(_) => None,
  //     Err(HumanNameError::LenCharMaxViolated) => {
  //       Some("The name you entered is too long.")
  //     }
  //     Err(HumanNameError::NotEmptyViolated) => Some("Your name is
  // required."),   }
  // };
  let name_hint = MaybeProp::derive(|| None);

  view! {
    <div class="flex-1" />
    <div class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8">
      <p class="title">"Create a Cache"</p>

      <p class="max-w-prose whitespace-pre-line">{ CACHE_DESCRIPTION }</p>

      <div class="h-0 border-t-[1.5px] border-base-6 w-full" />

      <InputField
        id="name" label_text="Cache Name" input_type="text" placeholder=""
        before=Some(InputFieldIcon::ArchiveBox)
        input_signal=read_name output_signal=write_name
        error_hint=name_hint autofocus=true
      />
    </div>
    <div class="flex-1" />
  }
}
