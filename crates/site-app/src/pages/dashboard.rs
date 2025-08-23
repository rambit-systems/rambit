mod cache;
mod entry;
mod requested_org;
mod store;

use leptos::prelude::*;

use self::{
  cache::CacheTable, entry::EntryTable, requested_org::RequestedOrgTile,
  store::StoreTable,
};

#[component]
pub fn DashboardPage() -> impl IntoView {
  view! {
    <div class="flex flex-col md:grid lg:grid-cols-[320px_minmax(0,_1fr)] gap-4 md:place-items-start">
      <p class="title lg:col-start-2">"Dashboard"</p>
      <RequestedOrgTile {..} class="md:place-self-end lg:place-self-auto md:w-80" />
      <div class="flex-1 md:col-span-2 lg:col-span-1 flex flex-col md:grid gap-4 md:grid-cols-2 md:place-self-stretch">
        <div class="col-span-2 p-6 elevation-flat flex flex-col gap-4">
          <EntryTable />
        </div>
        <div class="p-6 elevation-flat flex flex-col gap-4">
          <CacheTable />
        </div>
        <div class="p-6 elevation-flat flex flex-col gap-4">
          <StoreTable />
        </div>
      </div>
    </div>
  }
}
