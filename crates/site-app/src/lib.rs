#![feature(impl_trait_in_fn_trait_return)]
#![feature(iter_intersperse)]

mod components;
mod form_feedback_text;
mod formatting_utils;
mod hooks;
mod join_classes;
mod navigation;
mod pages;
mod reactive_utils;
mod resources;

use css_minify_macro::include_css;
use grid_state::AppState;
use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_router::{
  components::{ParentRoute, Route, Router, Routes},
  path,
};
use site_paddle::PaddleSetup;

use self::{components::org_selector::OrgSelectorRoutes, pages::*};

const PRELOAD_FONT_PATHS: &[&str] = &[
  "/fonts/funnel_sans/OpNIno8Dg9bX6Bsp3Wq69Tpyfhg.woff2",
  "/fonts/funnel_display/B50WF7FGv37QNVWgE0ga--4Pbb6dDYs.woff2",
];
const FAVICON_SVG: &str = include_str!("../public/favicon.svg");
const FAVICON_SVG_BASE64: &str =
  const_base::encode_as_str!(FAVICON_SVG, const_base::Config::B64);

pub fn shell() -> impl IntoView {
  let app_state = expect_context::<AppState>();
  let stylesheet = app_state.serve_config.inlined_stylesheet.clone();
  let htmx_path = format!("/dist/{}", app_state.serve_config.htmx_asset_name());

  QueryClient::new().provide();

  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />

        <script src={htmx_path}></script>

        { PRELOAD_FONT_PATHS.iter().map(|p| view! {
          <link rel="preload" href={*p} as="font" type="font/woff2" crossorigin="anonymous" />
        }).collect_view() }

        <style>{ stylesheet.as_ref() }</style>

        <style>{include_css!("style/fonts/funnel_sans.css")}</style>
        <style>{include_css!("style/fonts/funnel_display.css")}</style>
        <style>{include_css!("style/fonts/jetbrains_mono.css")}</style>

        <title>"Rambit Labs — Never waste another build"</title>

        <link
          rel="icon" type="image/svg+xml"
          href={format!("data:image/svg+xml;base64,{FAVICON_SVG_BASE64}")}
        />
      </head>
      <body>
        <PageContainer>
          <App/>
        </PageContainer>

        <PaddleSetup
          env={ app_state.domain.paddle_environment() }
          client_secret={ app_state.domain.paddle_client_secret() }
        />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  if use_context::<QueryClient>().is_none() {
    QueryClient::new().provide();
  }

  view! {
    <Router>
      <Routes fallback=|| "Page not found.".into_view()>
        <Route path=path!("") view=HomePage/>

        <DashboardPageRoutes />

        <Route path=path!("/org/:org/entry/:entry") view=protect_by_org(EntryPage) />

        <ParentRoute path=path!("/org/:org/settings") view=protect_by_org_owner(OrgSettingsPage)>
          <Route path=path!("/") view=OrgSettingsSubPageOverview />
          <Route path=path!("/billing") view=OrgSettingsSubPageBilling />
        </ParentRoute>

        <Route path=path!("/org/create_org") view=protect(CreateOrgPage) />
        <Route path=path!("/org/:org/create_cache") view=protect_by_org(CreateCachePage) />
        <Route path=path!("/org/:org/create_store") view=protect_by_org(CreateStorePage) />

        <SignupPageRoutes />
        <LoginPageRoutes />
        <Route path=path!("/auth/logout") view=LogoutPage />

        <OrgSelectorRoutes />

        <Route path=path!("/payment_link") view=PaymentLinkPage />
      </Routes>
    </Router>
  }
}

#[component]
fn PageContainer(children: Children) -> impl IntoView {
  view! {
    <main class="elevation-suppressed text-base-11 font-[450] text-base/[1.2]">
      <div class="page-container flex flex-col min-h-svh">
        <self::components::Navbar />
        { children() }
        <div class="flex-1" />
        <self::components::Footer />
      </div>
    </main>
  }
}
