use leptos::prelude::*;
use models::{dvf::RecordId, Org, PvStore};

use super::{DataTable, DataTableRefreshButton};
use crate::{
  components::StoreItemLink, resources::store::stores_in_org_query_scope,
};

#[island]
pub(super) fn StoreTable(org: RecordId<Org>) -> impl IntoView {
  let key_fn = move || org;
  let query_scope = stores_in_org_query_scope();

  view! {
    <div class="flex flex-row items-center gap-2">
      <p class="title">"Stores"</p>
      <div class="flex-1" />
      <DataTableRefreshButton
        key_fn=key_fn query_scope=query_scope.clone()
      />
    </div>

    <table class="table">
      <thead>
        <th>"Name"</th>
      </thead>
      <DataTable
        key_fn=key_fn query_scope=query_scope
        view_fn=move |c| view! {
          <tbody class="min-h-10">
            <For each=c key=|c| c.id children=|c| view! { <StoreDataRow store=c /> } />
          </tbody>
        }
      />
    </table>
  }
}

#[component]
fn StoreDataRow(store: PvStore) -> impl IntoView {
  view! {
    <tr>
      <th scope="row"><StoreItemLink id=store.id extra_class="btn-link-primary"/></th>
    </tr>
  }
}
