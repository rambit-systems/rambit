use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_router::hooks::use_params_map;
use models::{
  dvf::{RecordId, Visibility},
  Cache, Entry, PvCache, StorePath,
};

use crate::{
  components::{CacheItemLink, CopyButton, LockClosedHeroIcon},
  hooks::EntryHook,
  pages::UnauthorizedPage,
};

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
    <div class="flex flex-col gap-4">
      <TitleTile store_path={entry.store_path.clone()} />
      <div class="grid gap-4 grid-flow-col">
        <StorePathTile store_path={entry.store_path.clone()} />
        <CachesTile entry={entry.clone()} />
      </div>
    </div>
  }
}

#[component]
fn TitleTile(store_path: StorePath<String>) -> impl IntoView {
  let path = store_path.to_absolute_path();
  view! {
    <div class="p-6 elevation-flat flex flex-row gap-2 items-center">
      <p class="text-base-12 text-xl">
        { path.clone() }
      </p>
      <CopyButton copy_content={path} {..} class="size-6" />
    </div>
  }
}

#[component]
fn StorePathTile(store_path: StorePath<String>) -> impl IntoView {
  let string = store_path.to_string();
  let separator_index =
    string.find('-').expect("no separator found in store path");
  let (digest, _) = string.split_at(separator_index);
  let name = store_path.name().clone();

  const KEY_CLASS: &str = "place-self-end";
  const VALUE_CLASS: &str = "text-base-12 font-medium";

  let value_element = move |s: &str| {
    view! {
      <div class="flex flex-row gap-2 items-center">
        <p class=VALUE_CLASS>{ s }</p>
        <CopyButton
          copy_content={s.to_string()}
          {..} class="size-4"
        />
      </div>
    }
  };

  view! {
    <div class="p-6 elevation-flat flex flex-col gap-4">
      <p class="subtitle">
        "Store Path Breakdown"
      </p>
      <div class="grid gap-x-4 gap-y-1 grid-cols-[repeat(2,auto)]">
        <p class=KEY_CLASS>"Prefix"</p>
        { value_element("/nix/store/") }
        <p class=KEY_CLASS>"Digest"</p>
        { value_element(digest) }
        <p class=KEY_CLASS>"Name"</p>
        { value_element(&name) }
      </div>
    </div>
  }
}

#[component]
fn CachesTile(entry: Entry) -> impl IntoView {
  let caches = Signal::stored(entry.caches.clone());
  let store_path = Signal::stored(entry.store_path);
  view! {
    <div class="p-6 elevation-flat flex flex-col gap-4">
      <p class="subtitle">
        "Resident Caches"
      </p>

      <table class="table">
        <thead>
          <th>"Name"</th>
          <th>"Download Url"</th>
          <th>"Visibility"</th>
        </thead>
        <tbody class="animate-fade-in min-h-10">
          <For each=caches key=|r| *r children=move |r| view! {
            <CachesTileRow store_path=store_path cache_id=r />
          } />
        </tbody>
      </table>
    </div>
  }
}

#[component]
fn CachesTileRow(
  store_path: Signal<StorePath<String>>,
  cache_id: RecordId<Cache>,
) -> impl IntoView {
  let query_client = expect_context::<QueryClient>();

  let query_scope = crate::resources::cache::cache_query_scope();
  let resource = query_client.resource(query_scope, move || cache_id);

  let suspend = move || {
    Suspend::new(async move {
      match resource.await {
        Ok(Some(c)) => {
          view! { <CachesTileDataRow cache=c store_path=store_path /> }
            .into_any()
        }
        Ok(None) => None::<()>.into_any(),
        Err(e) => format!("Error: {e}").into_any(),
      }
    })
  };

  view! {
    <Transition fallback=|| ()>{ suspend }</Transition>
  }
}

#[component]
fn CachesTileDataRow(
  store_path: Signal<StorePath<String>>,
  cache: PvCache,
) -> impl IntoView {
  let download_url = format!(
    "/api/v1/c/{cache_name}/download/{store_path}",
    cache_name = cache.name,
    store_path = store_path(),
  );
  let vis_icon =
    matches!(cache.visibility, Visibility::Private).then_some(view! {
      <LockClosedHeroIcon {..} class="size-4 stroke-base-11/75 stroke-[2.0]" />
    });

  view! {
    <tr>
      <th scope="row">
        <CacheItemLink id=cache.id extra_class="text-link-primary"/>
      </th>
      <td>
        <a href={download_url} class="text-link text-link-primary">"Download"</a>
      </td>
      <td class="flex flex-row items-center gap-1">
        { cache.visibility.to_string() }
        { vis_icon }
      </td>
    </tr>
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
