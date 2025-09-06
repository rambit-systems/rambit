#![feature(impl_trait_in_fn_trait_return)]
#![feature(iter_intersperse)]

mod components;
mod hooks;
mod join_classes;
mod navigation;
mod pages;
mod reactive_utils;
mod resources;

use css_minify_macro::include_css;
use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_meta::{
  provide_meta_context, HashedStylesheet, MetaTags, Style, Title,
};
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};
use models::AuthUser;

use self::pages::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options={options.clone()} islands=true />

        <HashedStylesheet options id="leptos" />
        <Style>{include_css!("style/fonts/funnel_sans.css")}</Style>
        <Style>{include_css!("style/fonts/funnel_display.css")}</Style>
        <Style>{include_css!("style/fonts/jetbrains_mono.css")}</Style>

        <link rel="icon" href="/favicon.svg" type="image/svg+xml" />

        <MetaTags/>
      </head>
      <body>
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  QueryClient::new().provide();

  view! {
    <Title text="Rambit Labs — Never waste another build"/>

    <IslandContextProvider auth_user={ use_context() }>
      <Router>
        <PageContainer>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=path!("") view=HomePage/>
            <Route path=path!("/org/:org/dash") view=protect_by_org(DashboardPage) />
            <Route path=path!("/org/:org/entry/:entry") view=protect_by_org(EntryPage) />
            <Route path=path!("/org/:org/create_cache") view=protect_by_org(CreateCachePage) />
            <Route path=path!("/auth/signup") view=SignupPage/>
            <Route path=path!("/auth/login") view=LoginPage/>
            <Route path=path!("/auth/logout") view=LogoutPage/>
          </Routes>
        </PageContainer>
      </Router>
      // <LeptosFetchDevtools />
    </IslandContextProvider>
  }
}

#[allow(dead_code)]
#[island]
fn LeptosFetchDevtools() -> impl IntoView {
  let query_client = expect_context::<QueryClient>();
  view! {
    <leptos_fetch::QueryDevtools client=query_client/>
  }
}

#[component]
fn PageContainer(children: Children) -> impl IntoView {
  view! {
    <main class="elevation-suppressed text-base-11 font-normal text-base/[1.2]">
      <div class="page-container flex flex-col min-h-svh pb-8">
        <self::components::Navbar />
        { children() }
      </div>
    </main>
  }
}

fn protect_by_org<
  F: Fn() -> O + Send + Sync + Copy + 'static,
  O: IntoView + 'static,
>(
  func: F,
) -> impl Send + Clone + 'static + Fn() -> impl IntoAny {
  move || view! { <ProtectedByOrgPage> { func() } </ProtectedByOrgPage> }
}

#[island]
fn IslandContextProvider(
  auth_user: Option<AuthUser>,
  children: Children,
) -> impl IntoView {
  if let Some(auth_user) = auth_user {
    provide_context(auth_user);
  }
  QueryClient::new().provide();

  children()
}
