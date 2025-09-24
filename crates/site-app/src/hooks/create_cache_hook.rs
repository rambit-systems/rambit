use leptos::{ev::Event, prelude::*};
use leptos_fetch::QueryClient;
use models::{
  dvf::{EntityName, RecordId, StrictSlug, Visibility},
  Cache, Org,
};

use super::OrgHook;
use crate::{
  components::InputIcon, navigation::navigate_to,
  reactive_utils::touched_input_bindings,
};

pub struct CreateCacheHook {
  org_hook:              OrgHook,
  name_signal:           RwSignal<String>,
  visibility_signal:     RwSignal<Visibility>,
  sanitized_name_memo:   Memo<Option<EntityName>>,
  name_after_icon_memo:  Memo<Option<InputIcon>>,
  is_available_resource: LocalResource<Option<Result<bool, ServerFnError>>>,
  action:                ServerAction<CreateCache>,
}

impl CreateCacheHook {
  pub fn new() -> Self {
    let org_hook = OrgHook::new_requested();

    let name_signal = RwSignal::new(String::new());
    let sanitized_name_memo = Memo::new(move |_| {
      Some(EntityName::new(StrictSlug::new(name_signal())))
        .filter(|n| !n.to_string().is_empty())
    });
    let visibility_signal = RwSignal::new(Visibility::Private);

    let query_client = expect_context::<QueryClient>();
    let is_available_key_fn =
      move || sanitized_name_memo().map(|n| n.to_string());
    let is_available_query_scope =
      crate::resources::cache::cache_name_is_available_query_scope();
    let is_available_resource = expect_context::<QueryClient>()
      .local_resource(is_available_query_scope.clone(), is_available_key_fn);
    let is_available_fetching = query_client
      .subscribe_is_fetching(is_available_query_scope, is_available_key_fn);

    let name_after_icon_memo = Memo::new(move |_| {
      match (is_available_fetching(), is_available_resource.get()) {
        (true, _) => Some(InputIcon::Loading),
        (_, Some(Some(Ok(true)))) => Some(InputIcon::Check),
        (_, Some(Some(Ok(false)))) => Some(InputIcon::XMark),
        _ => None,
      }
    });

    let action = ServerAction::<CreateCache>::new();

    Self {
      org_hook,
      name_signal,
      visibility_signal,
      sanitized_name_memo,
      name_after_icon_memo,
      is_available_resource,
      action,
    }
  }

  pub fn name_after_icon(&self) -> Memo<Option<InputIcon>> {
    self.name_after_icon_memo
  }

  pub fn name_warn_hint(&self) -> Signal<Option<String>> {
    let (name_signal, sanitized_name_memo) =
      (self.name_signal, self.sanitized_name_memo);
    Signal::derive(move || {
      let (name, Some(sanitized_name)) =
        (name_signal.get(), sanitized_name_memo())
      else {
        return None;
      };
      if name != sanitized_name.clone().to_string() {
        return Some(format!(
          "This name will be converted to \"{sanitized_name}\"."
        ));
      }
      None
    })
  }

  pub fn name_error_hint(&self) -> Signal<Option<String>> {
    let (is_available_resource, sanitized_name_memo) =
      (self.is_available_resource, self.sanitized_name_memo);
    Signal::derive(move || {
      match (is_available_resource.get(), sanitized_name_memo()) {
        (Some(Some(Ok(false))), Some(sanitized_name)) => {
          Some(format!("The name \"{sanitized_name}\" is unavailable."))
        }
        (Some(Some(Err(_))), _) => {
          Some("Sorry, something went wrong.".to_owned())
        }
        _ => None,
      }
    })
  }

  pub fn name_bindings(&self) -> (Callback<(), String>, Callback<Event>) {
    touched_input_bindings(self.name_signal)
  }

  pub fn visibility_signal(&self) -> RwSignal<Visibility> {
    self.visibility_signal
  }

  pub fn show_spinner(&self) -> Signal<bool> {
    let (pending, value) = (self.action.pending(), self.action.value());
    // show if the action is loading or completed successfully
    Signal::derive(move || pending() || matches!(value.get(), Some(Ok(_))))
  }

  pub fn button_text(&self) -> Signal<&'static str> {
    let (pending, value) = (self.action.pending(), self.action.value());
    Signal::derive(move || match (value.get(), pending()) {
      // if the action is loading at all
      (_, true) => "Creating...",
      // if it's completed successfully
      (Some(Ok(_)), _) => "Redirecting...",
      // any other state
      _ => "Create Cache",
    })
  }

  pub fn action_trigger(&self) -> Callback<()> {
    let (
      org,
      visibility_signal,
      sanitized_name_memo,
      is_available_resource,
      action,
    ) = (
      self.org_hook.key(),
      self.visibility_signal,
      self.sanitized_name_memo,
      self.is_available_resource,
      self.action,
    );
    Callback::new(move |()| {
      // the name has been checked and is available
      if sanitized_name_memo().is_some()
        && matches!(is_available_resource.get(), Some(Some(Ok(true))))
      {
        action.dispatch_local(CreateCache {
          org:        org(),
          name:       sanitized_name_memo().unwrap().to_string(),
          visibility: visibility_signal(),
        });
      }
    })
  }

  pub fn create_redirect_effect(&self) -> Effect<LocalStorage> {
    let (dashboard_url, action) = (self.org_hook.dashboard_url(), self.action);
    Effect::new(move || {
      if matches!(action.value().get(), Some(Ok(_))) {
        navigate_to(&dashboard_url());
      }
    })
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
