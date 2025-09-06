//! CSS Minification Proc Macro
//!
//! This crate provides a proc macro that uses lightningcss to parse, minify,
//! and include CSS files at compile time.
//!
//! # Example
//!
//! ```rust,ignore
//! use css_minify_macro::include_css;
//!
//! // This will include the minified CSS as a string literal
//! let css = include_css!("path/to/styles.css");
//!
//! // Use in HTML templates
//! format!("<style>{}</style>", include_css!("styles.css"))
//! ```

use std::{env, fs, path::Path};

use lightningcss::stylesheet::{
  MinifyOptions, ParserOptions, PrinterOptions, StyleSheet,
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Error, LitStr, parse_macro_input};

/// Includes and minifies a CSS file at compile time.
///
/// This macro reads a CSS file, parses it with lightningcss, minifies it,
/// and returns the result as a string literal. The processing happens at
/// compile time, so there's no runtime overhead.
///
/// # Arguments
///
/// * `path` - A string literal containing the path to the CSS file, relative to
///   the crate root
///
/// # Examples
///
/// ```rust,ignore
/// use css_minify_macro::include_css;
///
/// // Include and minify a CSS file
/// let minified_css = include_css!("assets/styles.css");
///
/// // Use in HTML
/// let html = format!("<style>{}</style>", include_css!("styles.css"));
/// ```
///
/// # Errors
///
/// This macro will cause a compile-time error if:
/// - The file path is not a string literal
/// - The CSS file cannot be read
/// - The CSS cannot be parsed by lightningcss
/// - The CSS cannot be minified
#[proc_macro]
pub fn include_css(input: TokenStream) -> TokenStream {
  let lit_str = parse_macro_input!(input as LitStr);

  match include_css_impl(&lit_str) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

fn include_css_impl(lit_str: &LitStr) -> Result<TokenStream2, Error> {
  let css_path = lit_str.value();
  let span = lit_str.span();

  // Get the manifest directory (crate root)
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").map_err(|_| {
    Error::new(span, "Could not determine crate root directory")
  })?;

  // Construct the full path to the CSS file
  let full_path = Path::new(&manifest_dir).join(&css_path);

  // Read the CSS file
  let css_content = fs::read_to_string(&full_path).map_err(|e| {
    Error::new(
      span,
      format!("Could not read CSS file '{}': {}", full_path.display(), e),
    )
  })?;

  // Process the CSS with lightningcss
  let minified_css = process_css(&css_content, span)?;

  // Add the CSS file as a dependency so the build system knows to rebuild
  // when the CSS file changes
  let path_str = full_path.to_string_lossy();
  let dependency_hint = quote! {
      const _: &str = include_str!(#path_str);
  };

  // Return the minified CSS as a string literal
  Ok(quote! {
      {
          #dependency_hint
          #minified_css
      }
  })
}

fn process_css(css_content: &str, span: Span) -> Result<String, Error> {
  // Parse the stylesheet
  let mut stylesheet = StyleSheet::parse(css_content, ParserOptions::default())
    .map_err(|e| Error::new(span, format!("Failed to parse CSS: {e:?}")))?;

  // Minify the stylesheet
  stylesheet
    .minify(MinifyOptions::default())
    .map_err(|e| Error::new(span, format!("Failed to minify CSS: {e:?}")))?;

  // Serialize back to CSS string
  let result = stylesheet
    .to_css(PrinterOptions {
      minify: true,
      ..PrinterOptions::default()
    })
    .map_err(|e| Error::new(span, format!("Failed to serialize CSS: {e:?}")))?;

  Ok(result.code)
}
