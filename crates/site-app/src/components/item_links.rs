use std::iter::once;

use leptos::prelude::*;
use models::{dvf::RecordId, Cache, Store};

#[component]
pub fn CacheItemLink(
  id: RecordId<Cache>,
  #[prop(optional)] extra_class: Option<&'static str>,
) -> impl IntoView {
  let cache = crate::resources::cache::cache(move || id);
  let class = Signal::stored(
    once("text-link")
      .chain(extra_class)
      .collect::<Vec<_>>()
      .join(" "),
  );

  view! {
    <Suspense fallback=|| view! { <LoadingItemLink /> }>
      { move || match cache.get() {
        Some(Ok(Some(cache))) => {
          view! { <a class=class>{ cache.name.to_string() }</a> }.into_any()
        }
        Some(Ok(None) | Err(_)) => view! { <UnknownItemLink /> }.into_any(),
        None => view! { <LoadingItemLink /> }.into_any(),
      }}
    </Suspense>
  }
}

#[component]
pub fn StoreItemLink(
  id: RecordId<Store>,
  #[prop(optional)] extra_class: Option<&'static str>,
) -> impl IntoView {
  let store = crate::resources::store::store(move || id);
  let class = Signal::stored(
    once("text-link")
      .chain(extra_class)
      .collect::<Vec<_>>()
      .join(" "),
  );

  view! {
    <Suspense fallback=|| view! { <LoadingItemLink /> }>
      { move || match store.get() {
        Some(Ok(Some(store))) => {
          view! { <a class=class>{ store.name.to_string() }</a> }.into_any()
        }
        Some(Ok(None) | Err(_)) => view! { <UnknownItemLink /> }.into_any(),
        None => view! { <LoadingItemLink /> }.into_any(),
      }}
    </Suspense>
  }
}

#[component]
fn LoadingItemLink() -> impl IntoView {
  view! {
    <span>"[loading-item]"</span>
  }
}

#[component]
fn UnknownItemLink() -> impl IntoView {
  view! {
    <span>"[unknown-item]"</span>
  }
}
