use maud::{Markup, Render, html};
use models::{Cache, Org, RecordId, Store};

use crate::{
  components::misc_text::singular_or_plural,
  ctx::{Ctx, RequireAuth},
  hooks::OrgHook,
  indicators,
  resources::{
    AuthResult, fetch_cache, fetch_entry_count_for_cache,
    fetch_entry_count_for_store, fetch_org,
  },
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

pub fn cache_entry_count(
  ctx: Ctx<RequireAuth>,
  cache_id: RecordId<Cache>,
) -> Markup {
  ctx
    .suspend(
      move |ctx| async move {
        match ctx
          .fetch_cached(fetch_entry_count_for_cache, cache_id)
          .await
          .as_ref()
        {
          Ok(Some(AuthResult::Ok(c))) => html! {
            (c) " " (singular_or_plural(*c as usize, "entry", "entries"))
          },
          Ok(Some(AuthResult::Unauthorized)) => indicators::unauthorized(),
          Ok(None) => indicators::missing(),
          Err(_) => indicators::error(),
        }
      },
      indicators::loading(),
    )
    .render()
}

pub fn store_entry_count(
  ctx: Ctx<RequireAuth>,
  store_id: RecordId<Store>,
) -> Markup {
  ctx
    .suspend(
      move |ctx| async move {
        match ctx
          .fetch_cached(fetch_entry_count_for_store, store_id)
          .await
          .as_ref()
        {
          Ok(Some(AuthResult::Ok(c))) => html! {
            (c) " " (singular_or_plural(*c as usize, "entry", "entries"))
          },
          Ok(Some(AuthResult::Unauthorized)) => indicators::unauthorized(),
          Ok(None) => indicators::missing(),
          Err(_) => indicators::error(),
        }
      },
      indicators::loading(),
    )
    .render()
}
