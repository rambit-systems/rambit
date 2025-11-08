mod credentials_input;

use leptos::{prelude::*, server_fn::codec::Json};
use leptos_fetch::QueryClient;
use models::{
  EntityName, Org, R2StorageCredentials, RecordId, Store, StoreConfiguration,
};

use self::credentials_input::CredentialsInput;
use crate::{
  components::{form_layout::*, InputField, InputIcon, LoadingCircle},
  hooks::OrgHook,
  navigation::navigate_to,
  reactive_utils::touched_input_bindings,
};

const STORE_DESCRIPTION: &str =
  "A store represents a storage location for entries, for example an S3 \
   bucket. The store holds credentials for the storage location, and \
   configuration specifying how the entries it contains will be encoded.

   Stores are immutable aside from their entry list. To change a store's \
   credentials or encoding configuration, you will need to create a new store \
   and migrate the old store's entries to it. This incurs compute costs.";

#[island]
pub fn CreateStorePage() -> impl IntoView {
  let org_hook = OrgHook::new_requested();
  let org_key = org_hook.key();

  let name = RwSignal::new(String::new());
  let sanitized_name = Memo::new(move |_| {
    Some(EntityName::new(name())).filter(|n| !n.to_string().is_empty())
  });
  let (read_name, write_name) = touched_input_bindings(name);
  let credentials = RwSignal::<Option<R2StorageCredentials>>::new(None);
  let submit_touched = RwSignal::new(false);

  let is_available_query_scope =
    crate::resources::store::store_name_is_available_query_scope();
  let is_available_resource = expect_context::<QueryClient>()
    .local_resource(is_available_query_scope, move || {
      sanitized_name().map(|n| (org_key(), n.to_string()))
    });

  let action = ServerAction::<CreateStore>::new();
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
      Some(format!(
        "A store named \"{sanitized_name}\" already exists in this \
         organization."
      ))
    } else {
      None
    }
  });

  let org = org_hook.key();
  let submit_action = move |_| {
    submit_touched.set(true);

    // the name has been checked and is available
    if !(sanitized_name().is_some()
      && matches!(is_available_resource.get(), Some(Some(Ok(true)))))
    {
      return;
    }

    let Some(credentials) = credentials() else {
      return;
    };

    action.dispatch_local(CreateStore {
      org: org(),
      name: sanitized_name().unwrap().to_string(),
      credentials,
      configuration: StoreConfiguration {},
    });
  };

  let dashboard_url = org_hook.dashboard_url();
  Effect::new(move || {
    if matches!(action.value().get(), Some(Ok(_))) {
      navigate_to(&dashboard_url());
    }
  });

  const FORM_CLASS: &str = "p-8 self-stretch md:self-center md:w-2xl \
                            elevation-flat flex flex-col md:grid \
                            md:grid-cols-form gap-x-8 gap-y-12";

  view! {
    <div class=FORM_CLASS>
      <GridRowFull>
        <div class="flex flex-col gap-2">
          <p class="title">"Create a Store"</p>
          <p class="max-w-prose whitespace-pre-line">{ STORE_DESCRIPTION }</p>
        </div>
      </GridRowFull>

      <GridRowFull>
        <div class="h-0 border-t-[1.5px] border-base-6 w-full" />
      </GridRowFull>

      <GridRow>
        <GridRowLabel
          title="Store name"
          desc="Use something memorable."
        />
        <InputField
          id="name" label_text="" input_type="text" placeholder="Store Name"
          before={InputIcon::ArchiveBox}
          input_signal=read_name output_signal=write_name
          error_hint=name_error_hint warn_hint=name_warn_hint autofocus=true
        />
      </GridRow>

      <GridRow>
        <GridRowLabel
          title="Storage credentials"
          desc="These credentials are for the storage location where your data will sit."
        />
        <CredentialsInput signal=credentials show_hints={ move || submit_touched() } />
      </GridRow>

      <GridRow>
        <div />
        <label>
          <input type="submit" class="hidden" />
          <button
            class="btn btn-primary w-full max-w-80 justify-between"
            on:click=submit_action
          >
            <div class="size-4" />
            "Create Store"
            <LoadingCircle {..}
              class="size-4 transition-opacity"
              class=("opacity-0", move || { !loading() })
            />
          </button>
        </label>
      </GridRow>
    </div>
  }
}

#[server(prefix = "/api/sfn", input = Json)]
pub async fn create_store(
  org: RecordId<Org>,
  name: String,
  credentials: R2StorageCredentials,
  configuration: StoreConfiguration,
) -> Result<RecordId<Store>, ServerFnError> {
  use domain::DomainService;
  use models::StorageCredentials;

  crate::resources::authorize_for_org(org)?;

  let domain_service: DomainService = expect_context();

  let sanitized_name = EntityName::new(name.clone());
  if name != sanitized_name.clone().to_string() {
    return Err(ServerFnError::new("name is unsanitized"));
  }

  let store = Store {
    id: RecordId::new(),
    org,
    name: sanitized_name,
    credentials: StorageCredentials::R2(credentials),
    config: configuration,
  };

  domain_service.create_store(&store).await.map_err(|e| {
    tracing::error!("failed to create store: {e}");
    ServerFnError::new("internal error")
  })
}
