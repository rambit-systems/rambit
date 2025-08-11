use leptos::{either::Either, prelude::*};
use models::{dvf::RecordId, Cache, Store};

#[island]
pub fn CacheItemLink(id: RecordId<Cache>) -> impl IntoView {
  let cache = crate::resources::cache::cache(move || id);

  let suspend = move || {
    Suspend::new(async move {
      match cache.await {
        Ok(Some(cache)) => Either::Left(
          view! { <a class="text-link">{ cache.name.to_string() }</a> },
        ),
        Ok(None) | Err(_) => Either::Right(view! { <UnknownItemLink /> }),
      }
    })
  };

  view! {
    <span>
      <Suspense fallback=move || view! { <LoadingItemLink /> }>
        { suspend }
      </Suspense>
    </span>
  }
}

#[island]
pub fn StoreItemLink(id: RecordId<Store>) -> impl IntoView {
  let store = crate::resources::store::store(move || id);

  let suspend = move || {
    Suspend::new(async move {
      match store.await {
        Ok(Some(store)) => {
          Either::Left(view! { <a class="">{ store.name.to_string() }</a> })
        }
        Ok(None) | Err(_) => Either::Right(view! { <UnknownItemLink /> }),
      }
    })
  };

  view! {
    <span>
      <Suspense fallback=move || view! { <LoadingItemLink /> }>
        { suspend }
      </Suspense>
    </span>
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
