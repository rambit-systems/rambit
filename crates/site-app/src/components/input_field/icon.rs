use leptos::{ev::MouseEvent, prelude::*};

use crate::components::{
  icons::LockClosedHeroIcon, ArchiveBoxHeroIcon, EnvelopeHeroIcon, EyeHeroIcon,
  EyeSlashHeroIcon, GlobeAltHeroIcon, KeyHeroIcon, UserHeroIcon,
};

#[derive(Clone, Copy)]
pub enum InputIcon {
  ArchiveBox,
  Envelope,
  Eye,
  EyeSlash,
  GlobeAlt,
  Key,
  LockClosed,
  User,
}

macro_rules! icon_match {
  ($self_expr:expr, $click_handler:expr, $icon_class:expr, {
  $($variant:ident => $component:ident),* $(,)?
  }) => {
match $self_expr {
  $(
    InputIcon::$variant => view! {
      <$component {..} class=$icon_class on:click=$click_handler />
    }.into_any(),
  )*
}
  };
}

impl InputIcon {
  pub fn into_any(
    self,
    click_handler: Option<Callback<MouseEvent>>,
  ) -> AnyView {
    const ICON_CLASS: &str = "size-6";

    let click_handler = move |e| {
      if let Some(h) = click_handler {
        h.run(e)
      }
    };

    icon_match!(self, click_handler, ICON_CLASS, {
        ArchiveBox => ArchiveBoxHeroIcon,
        Envelope => EnvelopeHeroIcon,
        Eye => EyeHeroIcon,
        EyeSlash => EyeSlashHeroIcon,
        GlobeAlt => GlobeAltHeroIcon,
        Key => KeyHeroIcon,
        LockClosed => LockClosedHeroIcon,
        User => UserHeroIcon,
    })
  }
}
