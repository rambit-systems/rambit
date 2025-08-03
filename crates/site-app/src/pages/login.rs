use leptos::prelude::*;

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

#[component]
pub fn InputField(
  id: &'static str,
  label_text: &'static str,
  input_type: &'static str,
  placeholder: &'static str,
  #[prop(optional)] before: Option<Children>,
  #[prop(optional)] after: Option<Children>,
) -> impl IntoView {
  const OUTER_WRAPPER_CLASS: &str = "flex flex-col gap-1";
  const LABEL_CLASS: &str = "text-base-11";
  const INPUT_WRAPPER_CLASS: &str = "input-field max-w-80";
  const INPUT_CLASS: &str = "w-full py-2 focus-visible:outline-none";

  view! {
    <div class=OUTER_WRAPPER_CLASS>
      <label class=LABEL_CLASS for=id>{ label_text }</label>
      <div class=INPUT_WRAPPER_CLASS>
        { before.map(|b| b()) }
        <input
          class=INPUT_CLASS type=input_type
          placeholder=placeholder id=id
        />
        { after.map(|a| a()) }
      </div>
    </div>
  }
}

#[component]
pub fn EnvelopeHeroIcon() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="size-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75" />
    </svg>
  }
}

#[component]
pub fn LockClosedHeroIcon() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="size-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z" />
    </svg>
  }
}
