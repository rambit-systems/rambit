use leptos::prelude::*;
use leptos_meta::Script;
use models::PaddleClientSecret;

#[island]
pub fn PaddleProvider(children: Children) -> impl IntoView {
  let client_secret = expect_context::<PaddleClientSecret>();

  view! {
    <Script src="https://cdn.paddle.com/paddle/v2/paddle.js"></Script>
    <p>{ client_secret.0 }</p>
    { children() }
  }
}
