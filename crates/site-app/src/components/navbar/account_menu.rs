use leptos::prelude::*;
use models::AuthUser;

use crate::components::{Popover, PopoverContents, PopoverTrigger};

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
  let auth_user = expect_context::<AuthUser>();

  const POPOVER_CLASS: &str =
    "absolute right-0 top-[calc(100%+(var(--spacing)*4))] min-w-56 \
     elevation-lv1 p-2 flex flex-col gap-1 leading-none";

  view! {
    <div class=POPOVER_CLASS>
      <a class="btn-link btn-link-secondary">"Account Settings"</a>
      <a href="/auth/logout" class="btn btn-critical-subtle">"Log Out"</a>
    </div>
  }
}
