// use leptos::prelude::*;
// use leptos_fetch::QueryClient;
// use models::{
//   dvf::{RecordId, Visibility},
//   Cache, PvCache, Store,
// };

// use crate::resources::cache::cache_query_scope;

// #[derive(Clone)]
// pub struct CacheHook {
//   _key:     Callback<(), RecordId<Cache>>,
//   resource: Resource<Result<Option<PvCache>, ServerFnError>>,
// }

// impl CacheHook {
//   /// Creates a new [`CacheHook`].
//   pub fn new(
//     key: impl Fn() -> RecordId<Cache> + Copy + Send + Sync + 'static,
//   ) -> Self {
//     let client = expect_context::<QueryClient>();
//     let resource = client.resource(cache_query_scope(), key);

//     Self {
//       _key: Callback::new(move |_| key()),
//       resource,
//     }
//   }

//   pub fn all(&self) -> AsyncDerived<Result<Option<PvCache>, ServerFnError>> {
//     AsyncDerived::new({
//       let resource = self.resource;
//       move || async move { resource.await }
//     })
//   }

//   pub fn visibility(
//     &self,
//   ) -> AsyncDerived<Result<Option<Visibility>, ServerFnError>> {
//     AsyncDerived::new({
//       let resource = self.resource;
//       move || async move { resource.await.map(|o| o.map(|c| c.visibility)) }
//     })
//   }

//   pub fn default_store(
//     &self,
//   ) -> AsyncDerived<Result<Option<RecordId<Store>>, ServerFnError>> {
//     AsyncDerived::new({
//       let resource = self.resource;
//       move || async move { resource.await.map(|o| o.map(|c| c.default_store))
// }     })
//   }
// }
