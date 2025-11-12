use js_sys::{
  Object, Reflect,
  wasm_bindgen::{JsCast, JsValue},
};
use leptos::{
  logging::{error, log},
  prelude::*,
  web_sys::{self, js_sys},
};
use leptos_meta::Script;
use models::{
  AuthUser, PaddleClientSecret, PaddleCustomerId, PaddleEnvironment,
};

#[island]
pub fn PaddleProvider(children: Children) -> impl IntoView {
  Effect::new(|_| {
    let client_secret = expect_context::<PaddleClientSecret>();
    let environment = expect_context::<PaddleEnvironment>();
    let auth_user = use_context::<AuthUser>();
    let result = initialize_paddle(
      &client_secret,
      &environment,
      &auth_user.map(|au| au.customer_id),
    );
    if let Err(e) = result {
      error!("failed to initialize Paddle: {e:?}");
    }
  });

  view! {
    <Script src="https://cdn.paddle.com/paddle/v2/paddle.js"></Script>
    { children() }
  }
}

pub fn initialize_paddle(
  client_secret: &PaddleClientSecret,
  environment: &PaddleEnvironment,
  customer_id: &Option<PaddleCustomerId>,
) -> Result<(), JsValue> {
  log!("initializing Paddle");

  // get the global Paddle object
  let window = web_sys::window().expect("no global window");
  let paddle = Reflect::get(&window, &"Paddle".into())?;

  // set the environment
  if matches!(environment, PaddleEnvironment::Sandbox) {
    let env_object = Reflect::get(&paddle, &"Environment".into())?;
    let set_fn = Reflect::get(&env_object, &"set".into())?;
    let set_fn = set_fn.dyn_into::<js_sys::Function>()?;
    set_fn.call1(&env_object, &"sandbox".into())?;
  }

  // create the config object
  let config = Object::new();
  Reflect::set(
    &config,
    &"token".into(),
    &JsValue::from_str(&client_secret.0),
  )?;
  if let Some(customer_id) = customer_id.as_ref() {
    let pw_customer = Object::new();
    Reflect::set(
      &pw_customer,
      &"id".into(),
      &JsValue::from_str(customer_id.as_ref()),
    )?;
    Reflect::set(&config, &"pwCustomer".into(), &pw_customer.into())?;
    log!("using customer ID `{}` with Paddle", customer_id.as_ref());
  }

  // call Paddle.Initialize(config)
  let initialize_fn = Reflect::get(&paddle, &"Initialize".into())?;
  let initialize_fn = initialize_fn.dyn_into::<js_sys::Function>()?;
  initialize_fn.call1(&paddle, &config)?;

  log!("successfully initialized Paddle");

  Ok(())
}
