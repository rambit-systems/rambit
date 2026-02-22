// use leptos::prelude::*;
// use models::{PaddleClientSecret, PaddleEnvironment};
// use serde_json::to_string_pretty;

// #[component]
// pub fn PaddleSetup(
//   env: PaddleEnvironment,
//   client_secret: PaddleClientSecret,
// ) -> impl IntoView {
//   let environment_line = match env {
//     PaddleEnvironment::Sandbox => "Paddle.Environment.set(\"sandbox\");",
//     PaddleEnvironment::Production => "",
//   };
//   let config = serde_json::json!({
//     "token": client_secret.0,
//   });
//   let js = format!(
//     "{environment_line}\nPaddle.Initialize({config});",
//     config = to_string_pretty(&config).unwrap()
//   );

//   view! {
//     <script src="https://cdn.paddle.com/paddle/v2/paddle.js" />
//     <script>{ js }</script>
//   }
// }
