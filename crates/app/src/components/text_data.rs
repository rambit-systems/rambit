use maud::{Markup, Render};
use models::{Org, RecordId};

use crate::{
  ctx::{Ctx, RequireAuth},
  hooks::OrgHook,
  indicators,
};

pub fn org_descriptor(ctx: Ctx<RequireAuth>, org_id: RecordId<Org>) -> Markup {
  ctx
    .suspend(
      move |ctx| async move {
        let org = match ctx.fetch_org(org_id).await {
          Ok(Some(org)) => org,
          Ok(None) => return indicators::missing(),
          Err(_) => return indicators::error(),
        };

        let org_hook = OrgHook::new(org, ctx.auth_user());
        org_hook.descriptor()
      },
      indicators::loading(),
    )
    .render()
}
