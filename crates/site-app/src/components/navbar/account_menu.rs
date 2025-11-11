use leptos::prelude::*;
use models::AuthUser;

use crate::components::{
  ArrowRightStartOnRectangle, Cog6Tooth, Popover, PopoverContents,
  PopoverTrigger,
};

#[component]
fn AccountMenuTrigger() -> impl IntoView {
  let user = expect_context::<AuthUser>();

  const CLASS: &str = "size-10 flex flex-col justify-center items-center \
                       btn-secondary transition-colors rounded-full \
                       border-[1.5px] border-base-6 cursor-pointer";

  view! {
    <div class=CLASS>
      { user.name_abbr.to_string() }
    </div>
  }
}

#[island]
pub(crate) fn AccountMenu() -> impl IntoView {
  view! {
    <Popover>
      <PopoverTrigger slot>
        <AccountMenuTrigger />
      </PopoverTrigger>
      <PopoverContents slot>
        <AccountMenuMenu />
      </PopoverContents>
    </Popover>
  }
}

#[component]
fn AccountMenuMenu() -> impl IntoView {
  const POPOVER_CLASS: &str =
    "absolute right-0 top-[calc(100%+(var(--spacing)*4))] min-w-56 \
     elevation-lv1 z-50 p-2 flex flex-col gap-2 leading-none";

  view! {
    <div class=POPOVER_CLASS>
      <a class="btn-link btn-link-secondary btn-link-tight">
        <Cog6Tooth {..} class="size-5 stroke-base-11 stroke-[2.0]" />
        "Account Settings"
      </a>
      <a href="/auth/logout" class="btn btn-critical-subtle btn-tight">
        <ArrowRightStartOnRectangle {..} class="size-5 stroke-critical-11 stroke-[2.0]" />
        "Log Out"
      </a>
    </div>
  }
}
