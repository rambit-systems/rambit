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

      <p class="max-w-prose">
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
  let action = ServerAction::<Logout>::new();

  // loading represents both the action and the redirect, so we will continue
  // loading for the life of the page, if the action completed successfully
  let loading = {
    let (pending, value) = (action.pending(), action.value());
    move || pending() || matches!(value.get(), Some(Ok(_)))
  };

  let button_action = move |_| {
    action.dispatch_local(Logout {});
  };

  Effect::new(move || {
    if action.value().get() == Some(Ok(())) {
      navigate_to("/");
    }
  });

  view! {
    <button class="btn btn-critical-subtle w-full max-w-80 justify-between" on:click=button_action>
      <div class="size-4" />
      "Log out"
      <LoadingCircle {..}
        class="size-4 transition-opacity"
        class=("opacity-0", move || { !loading() })
      />
    </button>
  }
}

#[server(prefix = "/api/sfn")]
pub async fn logout() -> Result<(), ServerFnError> {
  let mut auth_session = expect_context::<auth_domain::AuthSession>();
  match auth_session.logout().await {
    Ok(_) => Ok(()),
    Err(e) => {
      tracing::error!("failed to deauthenticate: {e}");
      Err(ServerFnError::new("internal error"))
    }
  }
}
