use leptos::prelude::*;
use models::{dvf::RecordId, AuthUser, Org, PvOrg};

#[derive(Clone)]
pub struct UserActiveOrgHook {
  resource: Resource<Result<Option<PvOrg>, ServerFnError>>,
  user:     AuthUser,
}

impl UserActiveOrgHook {
  pub fn new(auth_user: AuthUser) -> Self {
    let user_orgs = auth_user.iter_orgs().collect::<Vec<_>>();
    let active_org = *user_orgs
      .get(auth_user.active_org_index as usize)
      .expect("active org index out of org list");
    let active_org_resource = crate::resources::org::org(move || active_org);

    UserActiveOrgHook {
      resource: active_org_resource,
      user:     auth_user,
    }
  }

  pub fn active_org_id(&self) -> RecordId<Org> {
    let orgs = self.user.iter_orgs().collect::<Vec<_>>();
    *orgs
      .get(self.user.active_org_index as usize)
      .unwrap_or_else(|| {
        leptos::logging::error!("active org index was out of bounds");
        orgs.last().expect("org list is empty")
      })
  }

  pub fn active_org_dash_url(&self) -> String {
    format!(
      "/dash/{active_org_id}",
      active_org_id = self.active_org_id()
    )
  }

  pub fn active_org_descriptor(&self) -> Memo<Option<String>> {
    let resource = self.resource;
    Memo::new({
      let user = self.user.clone();
      move |_| {
        resource.get().map(|r| {
          match r.map(|o| o.and_then(|o| o.user_facing_title(&user))) {
            Ok(Some(title)) => title,
            Ok(None) => "unknown-org".to_string(),
            Err(e) => {
              tracing::error!("failed to get org descriptor: {e}");
              "unknown-org".to_string()
            }
          }
        })
      }
    })
  }
}
