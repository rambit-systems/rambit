#![feature(impl_trait_in_fn_trait_return)]
#![feature(iter_intersperse)]

mod components;
mod formatting_utils;
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
  components::{ParentRoute, Route, Router, Routes},
  path,
};
use models::{AuthUser, PaddleClientSecret, PaddleEnvironment};

use self::pages::*;

const PRELOAD_FONT_PATHS: &[&str] = &[
  "/fonts/funnel_sans/OpNIno8Dg9bX6Bsp3Wq69Tpyfhg.woff2",
  "/fonts/funnel_display/B50WF7FGv37QNVWgE0ga--4Pbb6dDYs.woff2",
];
const FAVICON_SVG: &str = include_str!("../public/favicon.svg");
const FAVICON_SVG_BASE64: &str =
  const_base::encode_as_str!(FAVICON_SVG, const_base::Config::B64);

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options={options.clone()} islands=true />

        { PRELOAD_FONT_PATHS.iter().map(|p| view! {
          <link rel="preload" href={*p} as="font" type="font/woff2" crossorigin="anonymous" />
        }).collect_view() }

        <HashedStylesheet options id="leptos" />
        <Style>{include_css!("style/fonts/funnel_sans.css")}</Style>
        <Style>{include_css!("style/fonts/funnel_display.css")}</Style>
        <Style>{include_css!("style/fonts/jetbrains_mono.css")}</Style>

        <link
          rel="icon" type="image/svg+xml"
          href={format!("data:image/svg+xml;base64,{FAVICON_SVG_BASE64}")}
        />

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

    <IslandContextProvider
      auth_user={ use_context() }
      paddle_client_secret={ use_context() }
      paddle_environment={ use_context() }
    >
      <Router>
        <PageContainer>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=path!("") view=HomePage/>
            <Route path=path!("/org/:org/dash") view=protect_by_org(DashboardPage) />
            <Route path=path!("/org/:org/entry/:entry") view=protect_by_org(EntryPage) />
            <Route path=path!("/org/create_org") view=protect(CreateOrgPage) />
            <ParentRoute path=path!("/org/:org/settings") view=protect_by_org(OrgSettingsPage)>
              <Route path=path!("/") view=OrgSettingsSubPageOverview />
              <Route path=path!("/billing") view=OrgSettingsSubPageBilling />
            </ParentRoute>
            <Route path=path!("/org/:org/create_cache") view=protect_by_org(CreateCachePage) />
            <Route path=path!("/org/:org/create_store") view=protect_by_org(CreateStorePage) />
            <Route path=path!("/auth/signup") view=SignupPage />
            <Route path=path!("/auth/login") view=LoginPage />
            <Route path=path!("/auth/logout") view=LogoutPage />
            <Route path=path!("/payment_link") view=PaymentLinkPage />
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
    <main class="elevation-suppressed text-base-11 font-medium text-base/[1.2]">
      <div class="page-container flex flex-col min-h-svh">
        <self::components::Navbar />
        { children() }
        <div class="flex-1" />
        <self::components::Footer />
      </div>
    </main>
  }
}

#[island]
fn IslandContextProvider(
  auth_user: Option<AuthUser>,
  paddle_client_secret: Option<PaddleClientSecret>,
  paddle_environment: Option<PaddleEnvironment>,
  children: Children,
) -> impl IntoView {
  provide_meta_context();
  if let Some(auth_user) = auth_user {
    provide_context(auth_user);
  }
  if let Some(paddle_client_secret) = paddle_client_secret {
    provide_context(paddle_client_secret);
  }
  if let Some(paddle_environment) = paddle_environment {
    provide_context(paddle_environment);
  }
  QueryClient::new().provide();

  children()
}
