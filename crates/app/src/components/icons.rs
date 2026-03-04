use maud::{Markup, PreEscaped};

macro_rules! hero_icons {
  ($($name:ident => $file:literal),* $(,)?) => {
    $(
      pub fn $name() -> Markup {
        PreEscaped(include_str!(concat!("../../icons/", $file)).to_owned())
      }
    )*
  };
}

// make sure to remove any stroke, stroke-width, or class from the svg file

hero_icons! {
  user_hero_icon => "user.svg",
  envelope_hero_icon => "envelope.svg",
  lock_closed_hero_icon => "lock_closed.svg",
  chevron_down_hero_icon => "chevron_down.svg",
  check_hero_icon => "check.svg",
  cog_6_tooth_hero_icon => "cog_6_tooth.svg",
  plus_hero_icon => "plus.svg",
  building_office_2 => "building_office_2.svg",
  archive_box => "archive_box.svg",
  globe_alt => "globe_alt.svg",
  key_icon => "key.svg",
}

pub fn loading_circle() -> Markup {
  PreEscaped(include_str!("../../icons/loading_circle.svg").to_owned())
}
