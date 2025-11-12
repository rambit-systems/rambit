use js_sys::{
  Object, Reflect,
  wasm_bindgen::{JsCast, JsValue},
};
use leptos::{
  prelude::*,
  web_sys::{self, js_sys},
};
use leptos_meta::Script;
use models::PaddleClientSecret;

#[island]
pub fn PaddleProvider(children: Children) -> impl IntoView {
  #[cfg(feature = "hydrate")]
  {
    let client_secret = expect_context::<PaddleClientSecret>();
    match initialize_paddle(&client_secret) {
      Ok(()) => {
        leptos::logging::log!("successfully initialized Paddle");
      }
      Err(err) => {
        leptos::logging::error!("failed to initialize Paddle: {err:?}");
      }
    }
  }

  view! {
    <Script src="https://cdn.paddle.com/paddle/v2/paddle.js"></Script>
    { children() }
  }
}

pub fn initialize_paddle(
  client_secret: &PaddleClientSecret,
) -> Result<(), JsValue> {
  // get the global Paddle object
  let window = web_sys::window().expect("no global window");
  let paddle = Reflect::get(&window, &"Paddle".into())?;

  // create the config object
  let config = Object::new();
  Reflect::set(
    &config,
    &"token".into(),
    &JsValue::from_str(&client_secret.0),
  )?;

  // call Paddle.Initialize(config)
  let initialize_fn = Reflect::get(&paddle, &"Initialize".into())?;
  let initialize_fn = initialize_fn.dyn_into::<js_sys::Function>()?;
  initialize_fn.call1(&paddle, &config)?;

  Ok(())
}
