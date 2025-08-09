use leptos::prelude::*;
use models::{AuthUser, Org, OrgIdent};

use crate::resources;

#[derive(Clone)]
pub struct UserActiveOrgHook {
  resource: Resource<Result<Option<Org>, ServerFnError>>,
  user:     AuthUser,
}

impl UserActiveOrgHook {
  pub fn new(auth_user: AuthUser) -> Self {
    let user_orgs = auth_user.iter_orgs().collect::<Vec<_>>();
    let active_org = *user_orgs
      .get(auth_user.active_org_index as usize)
      .expect("active org index out of org list");
    let active_org_resource = resources::org(active_org);

    UserActiveOrgHook {
      resource: active_org_resource,
      user:     auth_user,
    }
  }

  pub fn active_org_descriptor(&self) -> Memo<Option<String>> {
    let resource = self.resource;
    let user_id = self.user.id;
    Memo::new(move |_| {
      resource
        .get()
        .map(|r| match r.map(|o| o.map(|o| o.org_ident)) {
          Ok(Some(OrgIdent::Named(entity_name))) => entity_name.to_string(),
          Ok(Some(OrgIdent::UserOrg(user_org_id)))
            if user_org_id == user_id =>
          {
            "Personal Org".to_owned()
          }
          Ok(Some(OrgIdent::UserOrg(user_org_id))) => {
            format!("{user_org_id}'s Org")
          }
          Ok(None) => "unknown-org".to_string(),
          Err(e) => {
            tracing::error!("failed to get org descriptor: {e}");
            "unknown-org".to_string()
          }
        })
    })
  }
}
