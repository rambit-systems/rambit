use leptos::{either::EitherOf11, prelude::*};

use crate::components::{
  icons::LockClosedHeroIcon, ArchiveBoxHeroIcon, ArrowPathHeroIcon,
  CheckHeroIcon, EnvelopeHeroIcon, EyeHeroIcon, EyeSlashHeroIcon,
  GlobeAltHeroIcon, KeyHeroIcon, UserHeroIcon, XMarkHeroIcon,
};

#[derive(Clone, Copy, PartialEq)]
pub enum InputIcon {
  ArchiveBox,
  Check,
  Envelope,
  Eye,
  EyeSlash,
  GlobeAlt,
  Key,
  Loading,
  LockClosed,
  User,
  XMark,
}

#[component]
pub fn InputIconComponent(icon: InputIcon) -> impl IntoView {
  let icon = match icon {
    InputIcon::ArchiveBox => EitherOf11::A(view! { <ArchiveBoxHeroIcon />}),
    InputIcon::Check => EitherOf11::B(view! { <CheckHeroIcon /> }),
    InputIcon::Envelope => EitherOf11::C(view! { <EnvelopeHeroIcon /> }),
    InputIcon::Eye => EitherOf11::D(view! { <EyeHeroIcon /> }),
    InputIcon::EyeSlash => EitherOf11::E(view! { <EyeSlashHeroIcon /> }),
    InputIcon::GlobeAlt => EitherOf11::F(view! { <GlobeAltHeroIcon /> }),
    InputIcon::Key => EitherOf11::G(view! { <KeyHeroIcon /> }),
    InputIcon::Loading => EitherOf11::H(view! { <ArrowPathHeroIcon /> }),
    InputIcon::LockClosed => EitherOf11::I(view! { <LockClosedHeroIcon /> }),
    InputIcon::User => EitherOf11::J(view! { <UserHeroIcon /> }),
    InputIcon::XMark => EitherOf11::K(view! { <XMarkHeroIcon /> }),
  };

  view! {
    <div class="size-6 shrink-0">
      { icon }
    </div>
  }
}
