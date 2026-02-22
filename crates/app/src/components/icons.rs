use maud::{Markup, PreEscaped};

// make sure to remove any stroke, stroke-width, etc. from the <svg> element

pub fn user_hero_icon() -> Markup {
  PreEscaped(include_str!("../../icons/user.svg").to_owned())
}
pub fn envelope_hero_icon() -> Markup {
  PreEscaped(include_str!("../../icons/envelope.svg").to_owned())
}
pub fn lock_closed_hero_icon() -> Markup {
  PreEscaped(include_str!("../../icons/lock_closed.svg").to_owned())
}

pub fn loading_circle() -> Markup {
  PreEscaped(include_str!("../../icons/loading_circle.svg").to_owned())
}
