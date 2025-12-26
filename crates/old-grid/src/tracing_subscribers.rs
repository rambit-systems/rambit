use std::any::Any;

use miette::IntoDiagnostic;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn setup_tracing() -> miette::Result<Box<dyn Any>> {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();

  #[cfg(not(feature = "json-tracing"))]
  let formatter = fmt::layer();
  #[cfg(feature = "json-tracing")]
  let formatter = fmt::layer().json();

  #[cfg(not(feature = "chrome-tracing"))]
  let (chrome_layer, chrome_guard) =
    (None::<tracing_chrome::ChromeLayer<_>>, Box::new(()));
  #[cfg(feature = "chrome-tracing")]
  let (chrome_layer, chrome_guard) = {
    let (layer, guard) = tracing_chrome::ChromeLayerBuilder::new().build();
    (Some(layer), Box::new(guard))
  };

  tracing_subscriber::registry()
    .with(formatter)
    .with(env_filter)
    .with(chrome_layer)
    .try_init()
    .into_diagnostic()?;

  Ok(chrome_guard)
}
