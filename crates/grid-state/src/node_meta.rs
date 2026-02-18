use miette::{Context, IntoDiagnostic};

/// Metadata for a node serving the grid service.
#[derive(Debug)]
pub struct NodeMeta {
  /// A string descriptor of the node's environment (dev, staging, prod, etc.).
  pub environment: String,
  /// The name of the node host.
  pub host_name:   String,
}

impl NodeMeta {
  /// Collect [`NodeMeta`] from the runtime environment.
  pub fn from_env() -> miette::Result<Self> {
    let env = std::env::var("GRID_ENV")
      .into_diagnostic()
      .context("`GRID_ENV` env var not populated")?;

    let host_name = gethostname::gethostname()
      .into_string()
      .map_err(|_| miette::miette!("hostname was not unicode"))?;

    Ok(NodeMeta {
      environment: env,
      host_name,
    })
  }
}
