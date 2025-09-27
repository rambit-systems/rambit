use leptos::prelude::*;

#[slot]
pub struct PopoverContents {
  children: Children,
}

#[slot]
pub struct PopoverTrigger {
  children: Children,
}

#[component]
pub fn Popover(
  popover_contents: PopoverContents,
  popover_trigger: PopoverTrigger,
) -> impl IntoView {
  let is_open = RwSignal::new(false);
  let toggle = move |_| {
    is_open.update(|open| {
      *open = !*open;
    })
  };

  let popover_ref = NodeRef::<leptos::html::Div>::new();

  // close on click outside
  #[cfg(feature = "hydrate")]
  let _ = leptos_use::on_click_outside(popover_ref, move |_| {
    if is_open() {
      is_open.set(false);
    }
  });

  // close on `Escape` key
  #[cfg(feature = "hydrate")]
  let _ = leptos_use::use_event_listener(
    leptos_use::use_window(),
    leptos::ev::keydown,
    move |evt| {
      if evt.key() == "Escape" && is_open.get() {
        is_open.set(false);
      }
    },
  );

  view! {
    <div class="relative">
      <div on:click=toggle>
        { (popover_trigger.children)() }
      </div>

      <div
        node_ref=popover_ref
        class="transition-opacity"
        class:opacity-0=move || !is_open()
        class:pointer-events-none=move || !is_open()
      >
        { (popover_contents.children)() }
      </div>
    </div>
  }
}
