use leptos::{either::EitherOf12, prelude::*};

use crate::components::{
  icons::LockClosedHeroIcon, ArchiveBoxHeroIcon, ArrowPathHeroIcon,
  BuildingOffice2HeroIcon, CheckHeroIcon, EnvelopeHeroIcon, EyeHeroIcon,
  EyeSlashHeroIcon, GlobeAltHeroIcon, KeyHeroIcon, UserHeroIcon, XMarkHeroIcon,
};

#[derive(Clone, Copy, PartialEq)]
pub enum InputIcon {
  ArchiveBox,
  BuildingOffice,
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
    InputIcon::ArchiveBox => EitherOf12::A(view! { <ArchiveBoxHeroIcon />}),
    InputIcon::BuildingOffice => {
      EitherOf12::B(view! { <BuildingOffice2HeroIcon /> })
    }
    InputIcon::Check => EitherOf12::C(view! { <CheckHeroIcon /> }),
    InputIcon::Envelope => EitherOf12::D(view! { <EnvelopeHeroIcon /> }),
    InputIcon::Eye => EitherOf12::E(view! { <EyeHeroIcon /> }),
    InputIcon::EyeSlash => EitherOf12::F(view! { <EyeSlashHeroIcon /> }),
    InputIcon::GlobeAlt => EitherOf12::G(view! { <GlobeAltHeroIcon /> }),
    InputIcon::Key => EitherOf12::H(view! { <KeyHeroIcon /> }),
    InputIcon::Loading => EitherOf12::I(view! { <ArrowPathHeroIcon /> }),
    InputIcon::LockClosed => EitherOf12::J(view! { <LockClosedHeroIcon /> }),
    InputIcon::User => EitherOf12::K(view! { <UserHeroIcon /> }),
    InputIcon::XMark => EitherOf12::L(view! { <XMarkHeroIcon /> }),
  };

  view! {
    <div class="size-6 shrink-0">
      { icon }
    </div>
  }
}
