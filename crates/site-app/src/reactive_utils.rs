use leptos::{ev::Event, prelude::*};

pub fn touched_input_bindings(
  s: RwSignal<String>,
) -> (impl Fn() -> String, impl Fn(Event)) {
  (
    move || s.get(),
    move |e| {
      s.set(event_target_value(&e));
    },
  )
}
