use leptos::prelude::*;

use crate::components::{EnvelopeHeroIcon, InputField, LockClosedHeroIcon};

#[component]
pub fn LoginPage() -> impl IntoView {
  view! {
    <div class="flex-1" />
    <div class="p-8 self-stretch md:self-center md:w-xl elevation-flat flex flex-col gap-8">
      <p class="title">"Login"</p>

      <div class="flex flex-col gap-4">
        <InputField
          id="email" label_text="Email Address"
          input_type="email" placeholder=""
          before={ Box::new(|| view!{ <EnvelopeHeroIcon /> }.into_any()) }
        />
        <InputField
          id="password" label_text="Password"
          input_type="password" placeholder=""
          before={ Box::new(|| view!{ <LockClosedHeroIcon /> }.into_any()) }
        />
      </div>

      <div class="flex flex-row gap-2">
        <button class="btn btn-primary">"Login"</button>
      </div>
    </div>
    <div class="flex-1" />
  }
}
