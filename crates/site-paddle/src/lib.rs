use leptos::prelude::*;
use leptos_meta::Script;

#[island]
pub fn PaddleProvider(children: Children) -> impl IntoView {
  view! {
    <Script src="https://cdn.paddle.com/paddle/v2/paddle.js"></Script>
    { children() }
  }
}
