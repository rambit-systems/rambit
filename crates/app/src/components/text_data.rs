use maud::{Markup, Render, html};
use models::{Cache, Org, RecordId};

use crate::{
  ctx::{Ctx, RequireAuth},
  hooks::OrgHook,
  indicators,
  resources::{fetch_cache, fetch_org},
};

pub fn org_descriptor(ctx: Ctx<RequireAuth>, org_id: RecordId<Org>) -> Markup {
  let suspend = ctx.suspend(
    move |ctx| async move {
      let org = match ctx.fetch_cached(fetch_org, org_id).await.as_ref() {
        Ok(Some(org)) => org.clone(),
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
      match ctx.fetch_cached(fetch_cache, cache_id).await.as_ref() {
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
