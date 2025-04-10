use std::str::FromStr;

use leptos::{either::Either, prelude::*};

use crate::{fetchers::*, utils::*};

#[component]
fn Store(#[prop(into)] store: Signal<models::Store>) -> impl IntoView {
  let store = Signal::derive(move || store.get());

  let store_id = Signal::derive(move || store.with(|c| c.id));

  let store_nickname =
    Signal::derive(move || store.with(|c| c.nickname.clone()));
  let store_config = move || store.with(|c| format!("{:#?}", c.credentials));

  view! {
    <Card>
      <TitleRow>
        <SuccessDot />
        <StoreIdTitleLink id=store_id />
      </TitleRow>
      <PropList>
        <KeyValue key="ID:">
          <StoreIdLink id=store_id />
        </KeyValue>
        <KeyValue key="Nickname:">
          <EntityNickname nickname=store_nickname />
        </KeyValue>
      </PropList>
      <div class="flex flex-row gap-2 items-start">
        <BoxHighlight> "Credentials:" </BoxHighlight>
        <CodeBlock> { store_config } </CodeBlock>
      </div>
    </Card>
  }
}

#[component]
pub fn StoreModelListPage() -> impl IntoView {
  let stores_resource = Resource::new(|| (), |_| fetch_all_stores());

  let stores_reader = move || {
    Suspend::new(async move {
      match stores_resource.await {
        Ok(stores) => {
          let stores = stores.into_iter().map(|c| view! { <Store store=c /> });
          Either::Left(view! {
            <ItemList items=stores />
          })
        }
        Err(e) => Either::Right(view! {
          <span>"Error: "{format!("{e}")}</span>
        }),
      }
    })
  };

  view! {
    <PageWrapper>
      <PageTitle level=1>"Store Model"</PageTitle>
      <p class="text-lg text-content2">"See the stores present in the database below."</p>
      <Suspense fallback=crate::fallback>
        { stores_reader }
      </Suspense>
    </PageWrapper>
  }
}

#[component]
pub fn StoreModelSinglePage() -> impl IntoView {
  let params = leptos_router::hooks::use_params_map();
  let id_param = params().get("id").unwrap_or_default();

  let store_id = match models::StoreRecordId::from_str(&id_param) {
    Ok(id) => id,
    Err(e) => {
      return Either::Left(view! {
        <div class="flex flex-col gap-4">
          <PageTitle level=1>"Store: Invalid ID"</PageTitle>
          <p class="text-lg text-content2">"Invalid store ID: " { e.to_string() }</p>
        </div>
      })
    }
  };

  let store_resource = Resource::new(move || store_id, fetch_store);

  let store_reader = move || {
    Suspend::new(async move {
      match store_resource.await {
        Ok(store) => Either::Left(view! {
          <Store store=store />
        }),
        Err(e) => Either::Right(view! {
          <p class="text-lg text-content2">"Error: "{format!("{e}")}</p>
        }),
      }
    })
  };

  Either::Right(view! {
    <PageWrapper>
      <PageTitle level=1>
        "Store: "
        <CodeHighlight>{ store_id.to_string() }</CodeHighlight>
      </PageTitle>
      <Suspense fallback=crate::fallback>
        { store_reader }
      </Suspense>
    </PageWrapper>
  })
}
