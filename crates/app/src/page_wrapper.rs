mod header;

use css_minify_macro::include_css;
use maud::{Markup, PreEscaped, html};

use self::header::header;
use crate::ctx::Ctx;

const PRELOAD_FONT_PATHS: &[&str] = &[
  "/fonts/funnel_sans/OpNIno8Dg9bX6Bsp3Wq69Tpyfhg.woff2",
  "/fonts/funnel_display/B50WF7FGv37QNVWgE0ga--4Pbb6dDYs.woff2",
];
const FAVICON_SVG: &str = include_str!("../public/favicon.svg");
const FAVICON_SVG_BASE64: &str =
  const_base::encode_as_str!(FAVICON_SVG, const_base::Config::B64);

pub(crate) fn page_wrapper(children: Markup, ctx: Ctx) -> Markup {
  let stylesheet = ctx.state().serve_config.inlined_stylesheet.clone();
  let htmx_asset_path =
    format!("/dist/{}", ctx.state().serve_config.htmx_asset_name());
  let svg_href = format!("data:image/svg+xml;base64,{FAVICON_SVG_BASE64}");

  let preload_fonts = PRELOAD_FONT_PATHS.iter().map(|p| html! {
    link rel="preload" href=(p) as="font" type="font/woff2" crossorigin="anonymous";
  });

  let header_element = header(ctx);

  html! {
    (maud::DOCTYPE)
    html lang="en" {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";

        script src=(htmx_asset_path) { }

        @for preload_font in preload_fonts {
          (preload_font)
        }

        style { (PreEscaped(stylesheet)) }

        style { (PreEscaped(include_css!("style/fonts/funnel_sans.css"))) }
        style { (PreEscaped(include_css!("style/fonts/funnel_display.css"))) }

        title { "Rambit by Porridge Co - Never waste another build" }

        link rel="icon" type="image/svg+xml" href=(svg_href);
      }
      body {
        main class="elevation-suppressed text-base-11 font-[450] text-base/[1.2]" {
          div class="page-container flex flex-col min-h-svh" {
            (header_element)
            (children)
            div class="flex-1" {}
          }
        }
      }
    }
  }
}
