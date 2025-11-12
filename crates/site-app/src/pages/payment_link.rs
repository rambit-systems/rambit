use leptos::prelude::*;
use site_paddle::PaddleProvider;

#[component]
pub fn PaymentLinkPage() -> impl IntoView {
  view! {
    <PaddleProvider>
      <div class="elevation-flat p-6 flex flex-col gap-4 h-full">
        <p class="title">"Payment Link"</p>
      </div>
    </PaddleProvider>
  }
}
