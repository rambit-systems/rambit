use leptos::prelude::*;
use leptos_fetch::QueryClient;
use models::{dvf::RecordId, AuthUser, Org, PvOrg};

use crate::resources::org::org_query_scope;

#[derive(Clone)]
pub struct OrgHook {
  key:      Callback<(), RecordId<Org>>,
  resource: Resource<Result<Option<PvOrg>, ServerFnError>>,
  user:     AuthUser,
}

impl OrgHook {
  /// Creates a new [`OrgHook`]. Requires [`AuthUser`] in context.
  pub fn new(
    key: impl Fn() -> RecordId<Org> + Copy + Send + Sync + 'static,
  ) -> Self {
    let auth_user = expect_context();
    let client = expect_context::<QueryClient>();
    let resource = client.resource(org_query_scope(), key);

    OrgHook {
      key: Callback::new(move |_| key()),
      resource,
      user: auth_user,
    }
  }

  /// Creates a new [`OrgHook`] using the [`AuthUser`]'s active org.
  pub fn new_active() -> Self {
    let auth_user = expect_context::<AuthUser>();
    let active_org = auth_user.active_org();
    Self::new(move || active_org)
  }

  pub fn dashboard_url(&self) -> Memo<String> {
    Memo::new({
      let key = self.key;
      {
        move |_| format!("/org/{}/dash", key.run(()))
      }
    })
  }

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
