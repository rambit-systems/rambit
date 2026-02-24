use maud::{Markup, html};

use crate::{ctx::Ctx, page_wrapper::page_wrapper};

pub fn unauthorized_page(ctx: Ctx) -> Markup {
  const CLASS: &str = "p-8 self-stretch md:self-center md:w-xl elevation-flat \
                       flex flex-col gap-8";

  let page = html! {
    div class=(CLASS) {
      p class="title" { "Unauthorized" }
      p {
        "Sorry, but you're not cleared to see this page."
      }
    }
  };

  page_wrapper(page, ctx)
}
