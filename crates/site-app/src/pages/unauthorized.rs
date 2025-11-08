use leptos::prelude::*;

#[component]
pub fn UnauthorizedPage() -> impl IntoView {
  const CONTAINER_CLASS: &str = "p-8 self-stretch md:self-center md:w-xl \
                                 elevation-flat flex flex-col gap-8";

  view! {
    <div class=CONTAINER_CLASS>
      <p class="title">"Unauthorized"</p>

      <p>
        "Sorry, but you're not cleared to see this page."
      </p>
    </div>
  }
}
