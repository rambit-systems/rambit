use leptos::prelude::*;
use models::{dvf::RecordId, Cache, Org};

#[component]
pub(super) fn CacheDashboardTile(org: RecordId<Org>) -> impl IntoView {
  let caches_in_org = crate::resources::cache::caches_in_org(move || org);
  let suspend = move || {
    Suspend::new(async move {
      match caches_in_org.await {
        Ok(caches) => view! { <CacheTable caches=caches /> }.into_any(),
        Err(e) => view! { { format!("Error: {e}") } }.into_any(),
      }
    })
  };

  view! {
    <div class="p-6 elevation-flat flex flex-col gap-4">
      <p class="title">"Caches"</p>

      <Suspense fallback=move || view! { "Loading..." }>
        { suspend }
      </Suspense>
    </div>
  }
}

#[component]
fn CacheTable(caches: Vec<Cache>) -> impl IntoView {
  view! {
    <table class="table">
      <thead>
        <th>"Name"</th>
        <th>"Visibility"</th>
        <th>"Default Cache"</th>
      </thead>
      <tbody>
        { move || caches.iter().map(|c| view! {
          <tr>
            <th scope="row">{ c.name.to_string() }</th>
            <td>{ c.visibility.to_string() }</td>
            <td>{ c.default_store.to_string() }</td>
          </tr>
        }).collect::<Vec<_>>() }
      </tbody>
    </table>
  }
}
