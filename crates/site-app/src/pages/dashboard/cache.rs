use leptos::prelude::*;
use models::{dvf::RecordId, Org, PvCache};

use super::{DataTable, DataTableRefreshButton};
use crate::{
  components::{CacheItemLink, StoreItemLink},
  resources::cache::caches_in_org_query_scope,
};

#[island]
pub(super) fn CacheTable(org: RecordId<Org>) -> impl IntoView {
  let key_fn = move || org;
  let query_scope = caches_in_org_query_scope();

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Caches"</p>
      <div class="flex-1" />
      <DataTableRefreshButton
        key_fn=key_fn query_scope=query_scope.clone()
      />
    </div>

    <table class="table">
      <thead>
        <th>"Name"</th>
        <th>"Visibility"</th>
        <th>"Default Store"</th>
      </thead>
      <DataTable
        key_fn=key_fn query_scope=query_scope
        view_fn=move |c| view! {
          <tbody class="min-h-10">
            <For each=c key=|c| c.id children=|c| view! { <CacheDataRow cache=c /> } />
          </tbody>
        }
      />
    </table>
  }
}

#[component]
fn CacheDataRow(cache: PvCache) -> impl IntoView {
  view! {
    <tr>
      <th scope="row"><CacheItemLink id=cache.id extra_class="btn-link-primary"/></th>
      <td>{ cache.visibility.to_string() }</td>
      <td><StoreItemLink id=cache.default_store /></td>
    </tr>
  }
}
