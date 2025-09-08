mod visibility_selector;

use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{
  dvf::{EntityName, RecordId, StrictSlug, Visibility},
  Cache, Org,
};

use self::visibility_selector::VisibilitySelector;
use crate::{
  components::{InputField, InputIcon, LoadingCircle},
  hooks::OrgHook,
  navigation::navigate_to,
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
  let org_hook = OrgHook::new_requested();

  let name = RwSignal::new(String::new());
  let sanitized_name = Memo::new(move |_| {
    Some(EntityName::new(StrictSlug::new(name())))
      .filter(|n| !n.to_string().is_empty())
  });
  let (read_name, write_name) = touched_input_bindings(name);
  let visibility = RwSignal::new(Visibility::Private);

  let is_available_query_scope =
    crate::resources::cache::cache_name_is_available_query_scope();
  let is_available_resource = expect_context::<QueryClient>()
    .local_resource(is_available_query_scope, move || {
      sanitized_name().map(|n| n.to_string())
    });

  let action = ServerAction::<CreateCache>::new();
  let loading = {
    let (pending, value) = (action.pending(), action.value());
    move || pending() || matches!(value.get(), Some(Ok(_)))
  };

  // error text for name field
  let name_warn_hint = MaybeProp::derive(move || {
    let (name, Some(sanitized_name)) = (name.get(), sanitized_name()) else {
      return None;
    };
    if name != sanitized_name.clone().to_string() {
      return Some(format!(
        "This name will be converted to \"{sanitized_name}\"."
      ));
    }
    None
  });
  let name_error_hint = MaybeProp::derive(move || {
    if let (Some(Some(Ok(false))), Some(sanitized_name)) =
      (is_available_resource.get(), sanitized_name())
    {
      Some(format!("The name \"{sanitized_name}\" is unavailable."))
    } else {
      None
    }
  });

  // submit callback
  let org = org_hook.key();
  let submit_action = move |_| {
    // the name has been checked and is available
    if sanitized_name().is_some()
      && matches!(is_available_resource.get(), Some(Some(Ok(true))))
    {
      action.dispatch_local(CreateCache {
        org:        org(),
        name:       sanitized_name().unwrap().to_string(),
        visibility: visibility(),
      });
    }
  };

  let dashboard_url = org_hook.dashboard_url();
  Effect::new(move || {
    if matches!(action.value().get(), Some(Ok(_))) {
      navigate_to(&dashboard_url());
    }
  });

  view! {
    <div class="flex-1" />
    <div class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8">
      <p class="title">"Create a Cache"</p>

      <p class="max-w-prose whitespace-pre-line">{ CACHE_DESCRIPTION }</p>

      <div class="h-0 border-t-[1.5px] border-base-6 w-full" />

      <div class="flex flex-col gap-4">
        <InputField
          id="name" label_text="Cache Name" input_type="text" placeholder=""
          before=Some(InputIcon::ArchiveBox)
          input_signal=read_name output_signal=write_name
          error_hint=name_error_hint warn_hint=name_warn_hint autofocus=true
        />

        <div class="flex flex-col gap-1">
          <p class="text-11-base">"Visibility"</p>
          <VisibilitySelector signal=visibility />
        </div>
      </div>

      <label class="flex flex-row gap-2">
        <input type="submit" class="hidden" />
        <button
          class="btn btn-primary w-full max-w-80 justify-between"
          on:click=submit_action
        >
          <div class="size-4" />
          "Create Cache"
          <LoadingCircle {..}
            class="size-4 transition-opacity"
            class=("opacity-0", move || { !loading() })
          />
        </button>
      </label>
    </div>
    <div class="flex-1" />
  }
}

#[server(prefix = "/api/sfn")]
pub async fn create_cache(
  org: RecordId<Org>,
  name: String,
  visibility: Visibility,
) -> Result<RecordId<Cache>, ServerFnError> {
  use prime_domain::PrimeDomainService;

  crate::resources::authorize_for_org(org)?;

  let prime_domain_service: PrimeDomainService = expect_context();

  let sanitized_name = EntityName::new(StrictSlug::new(name.clone()));
  if name != sanitized_name.clone().to_string() {
    return Err(ServerFnError::new("name is unsanitized"));
  }

  prime_domain_service
    .create_cache(org, sanitized_name, visibility)
    .await
    .map_err(|e| {
      tracing::error!("failed to create cache: {e}");
      ServerFnError::new("internal error")
    })
}
