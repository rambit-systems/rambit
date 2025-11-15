use std::panic::Location;

use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{AuthUser, Entry, Org, PvOrg, RecordId};

use crate::{pages::RequestedOrg, resources::org::org_query_scope};

/// A hook that provides data on an [`Org`]. Works in SSR.
///
/// `OrgHook` requires [`AuthUser`] and [`QueryClient`] as context. It is used
/// frequently enough that while it will panic if the required context cannot be
/// found, it will include its defining [`Location`] in the panic message.
///
/// It can target:
///  - the currently requested [`Org`] on org-scoped pages (pages with a `/:org`
///    segment)
///  - the user's [`active_org`](AuthUser::active_org)
///  - an arbitrary org
#[derive(Clone)]
pub struct OrgHook {
  #[cfg(debug_assertions)]
  defined_at: &'static Location<'static>,
  key:        Callback<(), RecordId<Org>>,
  resource:   Resource<Result<Option<PvOrg>, ServerFnError>>,
  user:       AuthUser,
}

impl OrgHook {
  fn new_inner(
    key: impl Fn() -> RecordId<Org> + Copy + Send + Sync + 'static,
    defined_at: &'static Location,
  ) -> Self {
    let auth_user = use_context().unwrap_or_else(|| {
      panic!(
        "context of type `AuthUser` was not found by `OrgHook` defined at {}",
        defined_at
      )
    });
    let client = use_context::<QueryClient>().unwrap_or_else(|| {
      panic!(
        "context of type `QueryClient` was not found by `OrgHook` defined at \
         {}",
        defined_at
      )
    });
    let resource = client.resource(org_query_scope(), key);

    OrgHook {
      #[cfg(debug_assertions)]
      defined_at,
      key: Callback::new(move |_| key()),
      resource,
      user: auth_user,
    }
  }

  /// Creates a new [`OrgHook`]. Requires [`AuthUser`] in context.
  #[track_caller]
  pub fn new(
    key: impl Fn() -> RecordId<Org> + Copy + Send + Sync + 'static,
  ) -> Self {
    Self::new_inner(key, Location::caller())
  }

  /// Creates a new [`OrgHook`] using the [`AuthUser`]'s active org.
  #[track_caller]
  pub fn new_active() -> Self {
    let defined_at = Location::caller();
    let auth_user = use_context::<AuthUser>().unwrap_or_else(|| {
      panic!(
        "context of type `AuthUser` was not found by `OrgHook` defined at \
         {defined_at}",
      )
    });
    let active_org = auth_user.active_org();
    Self::new_inner(move || active_org, defined_at)
  }

  /// Creates a new [`OrgHook`] using the [`RequestedOrg`] in context.
  #[track_caller]
  pub fn new_requested() -> Self {
    let defined_at = Location::caller();
    let RequestedOrg(requested_org) = use_context().unwrap_or_else(|| {
      panic!(
        "context of type `RequestedOrg` was not found by `OrgHook` defined at \
         {defined_at}",
      )
    });
    Self::new_inner(move || requested_org, defined_at)
  }

  pub fn key(
    &self,
  ) -> impl Fn() -> RecordId<Org> + Copy + Send + Sync + 'static {
    let key = self.key;
    move || key.run(())
  }

  /// The base URL for the org. No page exists here.
  fn base_url(&self) -> Memo<String> {
    Memo::new({
      let key = self.key;
      {
        move |_| format!("/org/{}", key.run(()))
      }
    })
  }

  /// The URL for the org's dashboard page, relative to the site root.
  pub fn dashboard_url(&self) -> Memo<String> {
    let base_url = self.base_url();
    Memo::new(move |_| format!("{}/dash", base_url()))
  }

  /// The URL for the org's "create cache" page, relative to the site root.
  pub fn create_cache_url(&self) -> Memo<String> {
    let base_url = self.base_url();
    Memo::new(move |_| format!("{}/create_cache", base_url()))
  }

  /// The URL for the org's "create store" page, relative to the site root.
  pub fn create_store_url(&self) -> Memo<String> {
    let base_url = self.base_url();
    Memo::new(move |_| format!("{}/create_store", base_url()))
  }

  /// The URL for the page of a given entry in the org, relative to the site
  /// root.
  pub fn entry_url(&self, entry_id: RecordId<Entry>) -> Memo<String> {
    let base_url = self.base_url();
    Memo::new(move |_| format!("{base}/entry/{entry_id}", base = base_url()))
  }

  /// The URL for the org's setting page, relative to the site root.
  pub fn settings_url(&self) -> Memo<String> {
    let base_url = self.base_url();
    Memo::new(move |_| format!("{}/settings", base_url()))
  }

  /// The canonical user-facing org name/descriptor.
  pub fn descriptor(&self) -> AsyncDerived<String> {
    AsyncDerived::new({
      let resource = self.resource;
      let auth_user = self.user.clone();
      move || {
        let auth_user = auth_user.clone();
        async move {
          resource
            .await
            .map(|o| {
              o.and_then(|o| o.user_facing_title(&auth_user))
                .unwrap_or("[unknown-org]".to_owned())
            })
            .unwrap_or("[error]".to_owned())
        }
      }
    })
  }
}

impl DefinedAt for OrgHook {
  fn defined_at(&self) -> Option<&'static Location<'static>> {
    #[cfg(debug_assertions)]
    {
      Some(self.defined_at)
    }
    #[cfg(not(debug_assertions))]
    {
      None
    }
  }
}
