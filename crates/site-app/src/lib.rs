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
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>
    <Title text="Welcome to Leptos"/>
    <Style>{include_str!("../style/funnel_sans.css")}</Style>
    <Style>{include_str!("../style/funnel_display.css")}</Style>

    <Router>
      <main>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage/>
        </Routes>
      </main>
    </Router>
  }
}

#[component]
fn OrbitButton(children: Children) -> impl IntoView {
  let base_btn_classes = "space-x-200 rtl:space-x-reverse";
  let primary_btn_classes =
    "bg-button-primary-background hover:bg-button-primary-background-hover \
     active:bg-button-primary-background-active \
     disabled:bg-button-primary-background \
     focus:bg-button-primary-background-focus text-button-primary-foreground \
     focus:text-button-primary-foreground-focus \
     active:text-button-primary-foreground-active \
     hover:text-button-primary-foreground-hover \
     disabled:text-button-primary-foreground active:shadow-button-active";
  let md_btn_size_classes = "px-button-padding-md";
  let rounded_btn_classes = "rounded-150 tb:rounded-100";

  let class = [
    base_btn_classes,
    primary_btn_classes,
    md_btn_size_classes,
    rounded_btn_classes,
  ]
  .join(" ");

  view! {
    <button class=class>
      { children() }
    </button>
  }
}

/// Renders the home page of your application.
#[island]
fn HomePage() -> impl IntoView {
  // Creates a reactive value to update the button
  let count = RwSignal::new(0);
  let on_click = move |_| *count.write() += 1;

  view! {
    <h1>"Welcome to Leptos!"</h1>
    <OrbitButton {..} on:click=on_click>"Click Me: " {count}</OrbitButton>
  }
}
