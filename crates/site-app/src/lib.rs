mod components;
mod pages;

use leptos::prelude::*;
use leptos_meta::{
  provide_meta_context, HashedStylesheet, MetaTags, Style, Title,
};
use leptos_router::{
  components::{Route, Router, Routes},
  path, StaticSegment,
};

use self::pages::{HomePage, LoginPage};

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
        <Style>{include_str!("../style/funnel_sans.css")}</Style>
        <Style>{include_str!("../style/funnel_display.css")}</Style>

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

  view! {
    <Title text="Rambit Labs — Never waste another build"/>

    <PageContainer>
      <Router>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage/>
          <Route path=path!("/auth/login") view=LoginPage/>
        </Routes>
      </Router>
    </PageContainer>
  }
}

#[component]
fn PageContainer(children: Children) -> impl IntoView {
  view! {
    <main class="elevation-suppressed text-base-11 font-medium">
      <div class="page-container flex flex-col gap-8 min-h-screen pb-8">
        <self::components::Navbar />
        { children() }
      </div>
    </main>
  }
}
