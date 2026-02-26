use maud::{Markup, PreEscaped, Render, html};
use models::{Org, RecordId};

use crate::{
  ctx::{Ctx, RequireAuth},
  hooks::OrgHook,
};

pub fn org_descriptor(ctx: Ctx<RequireAuth>, org_id: RecordId<Org>) -> Markup {
  let suspense = ctx.suspend(
    move |ctx| async move {
      let org = match ctx.fetch_org(org_id).await {
        Ok(Some(org)) => org,
        Ok(None) => return html! { "[unknown]" },
        Err(_) => return html! { "[error]" },
      };

      let org_hook = OrgHook::new(org, ctx.auth_user());
      let descriptor = org_hook.descriptor();
      PreEscaped(descriptor)
    },
    html! { "[loading]" },
  );

  suspense.render()
}
