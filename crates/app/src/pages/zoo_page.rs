//! Component zoo / playground page — shows all CSS components and utilities.

use axum::{Router, response::IntoResponse, routing::get};
use grid_state::AppState;
use maud::{Markup, html};

use crate::{
  components::icons::envelope_hero_icon, ctx::ResponseSeed,
  page_wrapper::page_wrapper,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn section(label: &str, content: Markup) -> Markup {
  html! {
    section class="flex flex-col gap-4" {
      p class="subtitle border-b border-base-6 pb-2" { (label) }
      (content)
    }
  }
}

fn tile(label: &str, content: Markup) -> Markup {
  html! {
    div class="flex flex-col gap-3" {
      p class="text-xs uppercase tracking-widest text-base-10 font-semibold" { (label) }
      div class="elevation-flat p-6 flex flex-row flex-wrap gap-3 items-center" {
        (content)
      }
    }
  }
}

// ---------------------------------------------------------------------------
// Sections
// ---------------------------------------------------------------------------

fn buttons_section() -> Markup {
  section("Buttons · .btn", html! {
    (tile("Normal", html! {
      button class="btn btn-primary" { "Primary" }
      button class="btn btn-primary-subtle" { "Primary subtle" }
      button class="btn btn-secondary" { "Secondary" }
      button class="btn btn-critical" { "Critical" }
      button class="btn btn-critical-subtle" { "Critical subtle" }
    }))
    (tile("Tight (.btn-tight)", html! {
      button class="btn btn-tight btn-primary" { "Primary" }
      button class="btn btn-tight btn-primary-subtle" { "Primary subtle" }
      button class="btn btn-tight btn-secondary" { "Secondary" }
      button class="btn btn-tight btn-critical" { "Critical" }
      button class="btn btn-tight btn-critical-subtle" { "Critical subtle" }
    }))
    (tile("Disabled", html! {
      button class="btn btn-primary" disabled? { "Primary" }
      button class="btn btn-primary-subtle" disabled? { "Primary subtle" }
      button class="btn btn-secondary" disabled? { "Secondary" }
      button class="btn btn-critical" disabled? { "Critical" }
      button class="btn btn-critical-subtle" disabled? { "Critical subtle" }
    }))
  })
}

fn btn_links_section() -> Markup {
  section("Button Links · .btn-link", html! {
    (tile("Normal", html! {
      button class="btn-link btn-link-primary" { "Primary" }
      button class="btn-link btn-link-secondary" { "Secondary" }
      button class="btn-link btn-link-critical" { "Critical" }
    }))
    (tile("Tight (.btn-link-tight)", html! {
      button class="btn-link btn-link-tight btn-link-primary" { "Primary" }
      button class="btn-link btn-link-tight btn-link-secondary" { "Secondary" }
      button class="btn-link btn-link-tight btn-link-critical" { "Critical" }
    }))
  })
}

fn input_field_section() -> Markup {
  section("Input Field · .input-field", html! {
    (tile("Text input", html! {
      div class="input-field" {
        input
          class="w-full py-2 focus-visible:outline-none"
          type="text"
          placeholder="Placeholder text";
      }
    }))
    (tile("Email input with icon", html! {
      div class="input-field" {
        div class="size-5 shrink-0" { (envelope_hero_icon()) }
        input
          class="w-full py-2 focus-visible:outline-none"
          type="email"
          placeholder="you@example.com";
      }
    }))
    (tile("Disabled", html! {
      div class="input-field" {
        input
          class="w-full py-2 focus-visible:outline-none"
          type="text"
          placeholder="Placeholder text"
          disabled?;
      }
    }))
  })
}

fn text_link_section() -> Markup {
  section("Text Link · .text-link", html! {
    (tile("Inline usage", html! {
      p {
        "Visit our "
        a class="text-link text-link-primary" href="#" { "documentation" }
        " for more details, or check out the "
        a class="text-link text-link-primary" href="#" { "source code" }
        "."
      }
    }))
  })
}

fn table_section() -> Markup {
  section("Table · .table", html! {
    (tile("Basic table", html! {
      div class="w-full overflow-x-auto" {
        table class="table" {
          thead class="table-header-group" {
            tr {
              th class="table-cell" { "Name" }
              th class="table-cell" { "Status" }
              th class="table-cell" { "Created" }
              th class="table-cell" { "Size" }
            }
          }
          tbody {
            tr class="table-row" {
              td class="table-cell" { "build-cache-main-abc123" }
              td class="table-cell" { "Active" }
              td class="table-cell" { "2025-03-01" }
              td class="table-cell" { "1.2 GB" }
            }
            tr class="table-row" {
              td class="table-cell" { "build-cache-feat-xyz987" }
              td class="table-cell" { "Evicted" }
              td class="table-cell" { "2025-02-28" }
              td class="table-cell" { "850 MB" }
            }
            tr class="table-row" {
              td class="table-cell" { "build-cache-ci-nightly" }
              td class="table-cell" { "Active" }
              td class="table-cell" { "2025-02-25" }
              td class="table-cell" { "3.4 GB" }
            }
          }
        }
      }
    }))
  })
}

fn typography_section() -> Markup {
  section("Typography", html! {
    (tile(".title", html! {
      p class="title" { "The quick brown fox" }
    }))
    (tile(".subtitle", html! {
      p class="subtitle" { "The quick brown fox" }
    }))
    (tile("Body (font-sans, base size)", html! {
      p class="font-sans" {
        "The quick brown fox jumps over the lazy dog. "
        "This is regular body copy at the default size and weight."
      }
    }))
    (tile("Display font (font-display)", html! {
      p class="font-display text-2xl tracking-tight" {
        "Never waste another build"
      }
    }))
    (tile("Monospace (font-mono)", html! {
      p class="font-mono text-sm" { "cargo build --release" }
    }))
  })
}

fn elevation_section() -> Markup {
  section("Elevation", html! {
    div class="flex flex-row flex-wrap gap-6" {
      (elevation_card("elevation-suppressed", "elevation-suppressed"))
      (elevation_card("elevation-flat", "elevation-flat"))
      (elevation_card("elevation-lv1", "elevation-lv1"))
      (elevation_card("elevation-lv2", "elevation-lv2"))
      (elevation_card("elevation-navbar", "elevation-navbar"))
    }
  })
}

fn elevation_card(class: &str, label: &str) -> Markup {
  html! {
    div class=(format!("{class} p-6 flex flex-col gap-1 min-w-40")) {
      p class="text-xs uppercase tracking-widest text-base-10 font-semibold" { (label) }
      p class="text-base-12 text-sm" { "Sample card" }
    }
  }
}

// ---------------------------------------------------------------------------
// Page handler
// ---------------------------------------------------------------------------

async fn zoo_page(ResponseSeed(ctx, resp): ResponseSeed) -> impl IntoResponse {
  let page = html! {
    div class="py-10 flex flex-col gap-12" {
      div class="flex flex-col gap-1" {
        p class="title" { "Component Zoo" }
        p class="text-base-11" {
          "A living reference for every CSS component and utility in the design system."
        }
      }

      (buttons_section())
      (btn_links_section())
      (input_field_section())
      (text_link_section())
      (table_section())
      (typography_section())
      (elevation_section())
    }
  };

  let document = page_wrapper(page, ctx);
  resp.into_stream(document)
}

pub fn sub_router() -> Router<AppState> {
  Router::new().route("/", get(zoo_page))
}
