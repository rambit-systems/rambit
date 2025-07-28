use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Style, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};

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

  const BASE_CLASSES: &str = "";

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

#[component]
fn Button(children: Children) -> impl IntoView {
  let class = "";

  view! {
    <button class=class>
      { children() }
    </button>
  }
}

#[component]
fn HomePage() -> impl IntoView {
  view! {
    <h1>"Welcome to Leptos!"</h1>
    <CounterButton />
  }
}

#[island]
fn CounterButton() -> impl IntoView {
  let count = RwSignal::new(0);
  let on_click = move |_| *count.write() += 1;

  view! {
    <Button {..} on:click=on_click>"Click Me: " {count}</Button>
  }
}
