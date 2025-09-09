use leptos::{ev::Event, prelude::*};

pub fn touched_input_bindings(
  s: RwSignal<String>,
) -> (Callback<(), String>, Callback<Event>) {
  (
    Callback::new(move |_| s.get()),
    Callback::new(move |e| {
      s.set(event_target_value(&e));
    }),
  )
}
