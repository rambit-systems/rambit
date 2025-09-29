mod caches_tile;
mod store_path_tile;
mod title_tile;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use models::{dvf::RecordId, Entry};

use self::{
  caches_tile::CachesTile, store_path_tile::StorePathTile,
  title_tile::TitleTile,
};
use crate::{hooks::EntryHook, pages::UnauthorizedPage};

#[component]
pub fn EntryPage() -> impl IntoView {
  let params = use_params_map();
  let requested_entry = params()
    .get("entry")
    .expect("missing entry path param")
    .parse::<RecordId<_>>()
    .ok();

  let Some(requested_entry) = requested_entry else {
    return view! { <UnauthorizedPage /> }.into_any();
  };

  let entry_hook = EntryHook::new(move || requested_entry);
  let intrensic = entry_hook.intrensic();
  let intrensic_suspend = move || {
    Suspend::new(async move {
      match intrensic.await {
        Ok(Some(entry)) => view! { <EntryInner entry=entry /> }.into_any(),
        Ok(None) => view! { <MissingEntryPage /> }.into_any(),
        Err(_) => view! { <ErrorEntryPage /> }.into_any(),
      }
    })
  };

  view! {
    <Suspense fallback=|| ()>{ intrensic_suspend }</Suspense>
  }
  .into_any()
}

#[component]
fn EntryInner(entry: Entry) -> impl IntoView {
  view! {
    <div class="flex flex-col md:grid md:grid-cols-[max-content_auto] gap-4">
      <div class="hidden md:block"/>
      <TitleTile store_path={entry.store_path.clone()} />
      <ActionTile />
      <div class="flex flex-row gap-4 flex-wrap">
        <StorePathTile store_path={entry.store_path.clone()} />
        <CachesTile entry={entry.clone()} />
      </div>
    </div>
  }
}

#[component]
fn ActionTile() -> impl IntoView {
  view! {
    <div class="md:w-64 lg:w-80 p-6 elevation-flat flex flex-col gap-4 align-self-start">
      <p class="subtitle">"Actions"</p>
      <div class="flex flex-col gap-2">
        <button class="btn btn-critical">
          "Delete Entry"
        </button>
      </div>
    </div>
  }
}

#[component]
fn MissingEntryPage() -> impl IntoView {
  view! {
    <div class="p-6 elevation-flat flex flex-col gap-4 items-center">
      <p class="title">
        "( ˶°ㅁ°) !!"
        " We don't have that entry"
      </p>
      <p>"Looks like we don't have that entry! Try uploading it through the CLI."</p>
    </div>
  }
}

#[component]
fn ErrorEntryPage() -> impl IntoView {
  view! {
    <div class="p-6 elevation-flat flex flex-col gap-4 items-center">
      <p class="title">
        "ヽ(°〇°)ﾉ"
        " Something went wrong"
      </p>
      <p>"Looks like something went wrong when finding your entry. We apologize!"</p>
    </div>
  }
}
