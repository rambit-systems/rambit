mod components;
mod context;
mod join_classes;
mod navigation;
mod pages;
mod reactive_utils;
mod resources;

use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_meta::{
  provide_meta_context, HashedStylesheet, MetaTags, Style, Title,
};
use leptos_router::{
  components::{Route, Router, Routes},
  path, StaticSegment,
};

use self::{
  context::UserDataContextProvider,
  pages::{DashboardPage, HomePage, LoginPage, LogoutPage, SignupPage},
};

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
        <Style>{include_str!("../style/fonts/funnel_sans.css")}</Style>
        <Style>{include_str!("../style/fonts/funnel_display.css")}</Style>

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

    <Router>
      <UserDataContextProvider>
        <PageContainer>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=StaticSegment("") view=HomePage/>
            <Route path=path!("/dash") view=DashboardPage />
            <Route path=path!("/auth/signup") view=SignupPage/>
            <Route path=path!("/auth/login") view=LoginPage/>
            <Route path=path!("/auth/logout") view=LogoutPage/>
          </Routes>
        </PageContainer>
      </UserDataContextProvider>
    </Router>
  }
}

#[component]
fn PageContainer(children: Children) -> impl IntoView {
  view! {
    <main class="elevation-suppressed text-base-11 font-medium">
      <div class="page-container flex flex-col min-h-svh pb-8">
        <self::components::Navbar />
        { children() }
      </div>
    </main>
  }
}
