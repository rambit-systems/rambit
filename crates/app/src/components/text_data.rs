use maud::{Markup, Render, html};
use models::{Cache, Org, RecordId};

use crate::{
  ctx::{Ctx, RequireAuth},
  hooks::OrgHook,
  indicators,
};

pub fn org_descriptor(ctx: Ctx<RequireAuth>, org_id: RecordId<Org>) -> Markup {
  let suspend = ctx.suspend(
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
  );
  suspend.render()
}

pub fn cache_link(ctx: Ctx<RequireAuth>, cache_id: RecordId<Cache>) -> Markup {
  let suspend = ctx.suspend(
    move |ctx| async move {
      match ctx.state().domain.meta().fetch_cache_by_id(cache_id).await {
        Ok(Some(c)) => html! {
          a class="text-link text-link-primary" {
            (c.name.to_string())
          }
        },
        Ok(None) => indicators::missing(),
        Err(_) => indicators::error(),
      }
    },
    indicators::loading(),
  );
  suspend.render()
}
