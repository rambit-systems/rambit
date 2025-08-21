use leptos::prelude::*;

use crate::{components::LoadingCircle, navigation::navigate_to};

#[component]
pub fn LogoutPage() -> impl IntoView {
  view! {
    <div class="flex-1" />
    <div
      class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8"
    >
      <p class="title">"Log out"</p>

      <p>
        "Are you sure you want to log out? We're sad to see you go but excited for you to come back."
      </p>

      <div class="flex flex-row">
        <LogoutButton />
      </div>
    </div>
    <div class="flex-1" />
  }
}

#[island]
pub fn LogoutButton() -> impl IntoView {
  let action = Action::new_local(move |(): &()| async move {
    let resp = gloo_net::http::Request::post("/api/v1/deauthenticate")
      .send()
      .await
      .map_err(|e| format!("request error: {e}"))?;

    match resp.status() {
      200 => Ok(true),
      401 => Ok(false),
      400 => Err(format!("response error: {}", resp.text().await.unwrap())),
      s => Err(format!("status error: got unknown status {s}")),
    }
  });

  // loading represents both the action and the redirect, so we will continue
  // loading for the life of the page, if the action completed successfully
  let loading = {
    let (pending, value) = (action.pending(), action.value());
    move || pending() || matches!(value.get(), Some(Ok(true)))
  };

  let button_action = move |_| {
    action.dispatch_local(());
  };

  Effect::new(move || {
    if action.value().get() == Some(Ok(true)) {
      navigate_to("/");
    }
  });

  view! {
    <button class="btn btn-critical w-full max-w-80" on:click=button_action>
      <div class="flex-1" />
      <div class="flex-1 flex flex-row justify-center items-center">
        "Log out"
      </div>
      <div class="flex-1 flex flex-row justify-end items-center">
        <LoadingCircle {..}
          class="size-4 transition-opacity"
          class=("opacity-0", move || { !loading() })
        />
      </div>
    </button>
  }
}
