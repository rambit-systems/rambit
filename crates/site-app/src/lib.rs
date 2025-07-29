mod pages;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Style, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};

use self::pages::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <AutoReload options=options.clone() />
        <HydrationScripts options islands=true/>
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

  const BASE_CLASSES: &str =
    "bg-cloud-light text-ink-dark border-cloud-normal font-medium";

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>
    <Title text="Welcome to Leptos"/>
    <Style>{include_str!("../style/funnel_sans.css")}</Style>
    <Style>{include_str!("../style/funnel_display.css")}</Style>

    <Router>
      <main class=BASE_CLASSES>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage/>
        </Routes>
      </main>
    </Router>
  }
}
