mod cache;
mod current_org;
mod entry;
mod store;

use leptos::prelude::*;
use models::{dvf::RecordId, Org};

use self::{
  cache::CacheTable, current_org::CurrentOrgTile, entry::EntryTable,
  store::StoreTable,
};

#[component]
pub fn DashboardPage() -> impl IntoView {
  let org: RecordId<Org> = expect_context();

  view! {
    <div class="grid grid-cols-[320px_minmax(0,_1fr)] gap-4 place-items-start">
      <p class="title col-start-2">"Dashboard"</p>
      <CurrentOrgTile org={org} {..} class="w-80" />
      <div class="flex-1 grid gap-4 grid-cols-2 place-self-stretch">
        <div class="col-span-2 p-6 elevation-flat flex flex-col gap-4">
          <EntryTable org=org />
        </div>
        <div class="p-6 elevation-flat flex flex-col gap-4">
          <CacheTable org=org />
        </div>
        <div class="p-6 elevation-flat flex flex-col gap-4">
          <StoreTable org=org />
        </div>
      </div>
    </div>
  }
}
