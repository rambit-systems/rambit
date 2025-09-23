use leptos::{ev::Event, prelude::*};
use leptos_fetch::QueryClient;
use models::{
  dvf::{EntityName, RecordId, StrictSlug},
  Org,
};

use super::OrgHook;
use crate::{
  components::InputIcon, navigation::navigate_to,
  reactive_utils::touched_input_bindings,
};

pub struct CreateOrgHook {
  name_signal:           RwSignal<String>,
  sanitized_name_memo:   Memo<Option<EntityName>>,
  name_after_icon_memo:  Memo<Option<InputIcon>>,
  is_available_resource: LocalResource<Option<Result<bool, ServerFnError>>>,
  action:                ServerAction<CreateOrg>,
}

impl CreateOrgHook {
  pub fn new() -> Self {
    let name_signal = RwSignal::new(String::new());
    let sanitized_name_memo = Memo::new(move |_| {
      Some(EntityName::new(StrictSlug::new(name_signal())))
        .filter(|n| !n.to_string().is_empty())
    });

    let query_client = expect_context::<QueryClient>();
    let is_available_key_fn =
      move || sanitized_name_memo().map(|n| n.to_string());
    let is_available_query_scope =
      crate::resources::org::org_name_is_available_query_scope();
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

    let action = ServerAction::<CreateOrg>::new();

    Self {
      name_signal,
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

  pub fn show_spinner(&self) -> Signal<bool> {
    let (pending, value) = (self.action.pending(), self.action.value());
    // show if the action is loading or completed successfully
    Signal::derive(move || pending() || matches!(value.get(), Some(Ok(_))))
  }

  pub fn action_trigger(&self) -> Callback<()> {
    let (sanitized_name_memo, is_available_resource, action) = (
      self.sanitized_name_memo,
      self.is_available_resource,
      self.action,
    );
    Callback::new(move |()| {
      // the name has been checked and is available
      if sanitized_name_memo().is_some()
        && matches!(is_available_resource.get(), Some(Some(Ok(true))))
      {
        action.dispatch_local(CreateOrg {
          name: sanitized_name_memo().unwrap().to_string(),
        });
      }
    })
  }

  pub fn create_redirect_effect(&self) -> Effect<LocalStorage> {
    let action = self.action;
    Effect::new(move || {
      if let Some(Ok(id)) = action.value().get() {
        let org_hook = OrgHook::new(move || id);
        navigate_to(&org_hook.dashboard_url()());
      }
    })
  }
}

#[server(prefix = "/api/sfn")]
pub async fn create_org(name: String) -> Result<RecordId<Org>, ServerFnError> {
  use auth_domain::AuthDomainService;
  use prime_domain::PrimeDomainService;

  let auth_user = crate::resources::authenticate()?;

  let prime_domain_service: PrimeDomainService = expect_context();
  let auth_domain_service: AuthDomainService = expect_context();

  let sanitized_name = EntityName::new(StrictSlug::new(name.clone()));
  if name != sanitized_name.clone().to_string() {
    return Err(ServerFnError::new("name is unsanitized"));
  }

  let org = prime_domain_service
    .create_org(sanitized_name)
    .await
    .map_err(|e| {
      tracing::error!("failed to create org: {e}");
      ServerFnError::new("internal error")
    })?;

  prime_domain_service
    .add_org_to_user(auth_user.id, org)
    .await
    .map_err(|e| {
      tracing::error!("failed to add org to user: {e}");
      ServerFnError::new("internal error")
    })?;

  auth_domain_service
    .switch_active_org(auth_user.id, org)
    .await
    .map_err(|e| {
      tracing::error!("failed to fetch org: {e}");
      ServerFnError::new("internal error")
    })?;

  Ok(org)
}
