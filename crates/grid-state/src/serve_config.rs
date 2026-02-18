use std::{io::Read, path::PathBuf, sync::Arc};

use miette::{Context, IntoDiagnostic};
use tracing::warn;

/// Configuration for serving HTTP responses.
#[derive(Debug)]
pub struct ServeConfig {
  /// The directory to serve static assets out of.
  pub static_asset_dir:   PathBuf,
  /// The stylesheet to be inlined into the doc `<head>`.
  pub inlined_stylesheet: Arc<str>,
  /// Whether to serve the non-minified version of the HTMX lib.
  pub non_minified_htmx:  bool,
}

impl ServeConfig {
  /// Build [`ServeConfig`] from the runtime environment.
  pub fn build() -> miette::Result<Self> {
    let static_asset_dir = std::env::var("GRID_STATIC_ASSET_DIR")
      .into_diagnostic()
      .context("`GRID_STATIC_ASSET_DIR` env var not populated")?;
    let static_asset_dir = PathBuf::from(static_asset_dir);

    let stylesheet_path = std::env::var("GRID_STYLESHEET_PATH")
      .into_diagnostic()
      .context("`GRID_STYLESHEET_PATH` env var not populated")?;
    let mut stylesheet_content = String::new();
    match std::fs::File::open(&stylesheet_path) {
      Ok(mut f) => {
        f.read_to_string(&mut stylesheet_content)
          .into_diagnostic()
          .context("failed to read from stylesheet file")?;
      }
      Err(e) => {
        tracing::warn!("failed to open stylesheet file: {e}");
      }
    };
    let stylesheet_content = Arc::<str>::from(stylesheet_content);

    let non_minified_htmx = std::env::var("GRID_NO_MINIFY_HTMX")
      .ok()
      .and_then(|val| match val.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" | "" => Some(false),
        v => {
          warn!(
            "did not recognize value for env var `GRID_NO_MINIFY_HTMX`, \
             ignoring: {v}"
          );
          None
        }
      })
      .unwrap_or(false);

    Ok(ServeConfig {
      static_asset_dir,
      inlined_stylesheet: stylesheet_content,
      non_minified_htmx,
    })
  }

  /// The name of the HTMX asset to serve.
  pub fn htmx_asset_name(&self) -> &'static str {
    match self.non_minified_htmx {
      true => "htmx.js",
      false => "htmx.min.js",
    }
  }
}
