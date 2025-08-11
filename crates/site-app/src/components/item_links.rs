use leptos::{either::Either, prelude::*};
use models::{dvf::RecordId, Cache};

#[component]
pub fn CacheItemLink(id: RecordId<Cache>) -> impl IntoView {
  let cache = crate::resources::cache(id);

  let suspend = move || {
    Suspend::new(async move {
      match cache.await {
        Ok(Some(cache)) => {
          Either::Left(view! { <a class="">{ cache.name.to_string() }</a> })
        }
        Ok(None) | Err(_) => Either::Right(view! { <UnknownItemLink /> }),
      }
    })
  };

  view! {
    <Suspense fallback=move || view! { <LoadingItemLink /> }>
      { suspend }
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
