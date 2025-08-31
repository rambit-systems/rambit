use std::fmt;

use miette::{Context, IntoDiagnostic, ensure, miette};
use tokio::process::Command;

pub(crate) struct CurrentSystem(String);

impl fmt::Display for CurrentSystem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl CurrentSystem {
  pub(crate) async fn calculate() -> miette::Result<Self> {
    tracing::debug!("getting current system");

    let mut command = Command::new("nix-instantiate");

    command.args(["--eval", "-E", "builtins.currentSystem"]);

    let output = command
      .output()
      .await
      .into_diagnostic()
      .context("failed to spawn nix current-system process")?;

    output
      .status
      .exit_ok()
      .into_diagnostic()
      .context("nix current-system process exited with failure status")?;

    let output_string = output
      .stdout
      .utf8_chunks()
      .map(|c| c.valid())
      .filter(|c| !c.is_empty())
      .collect::<Vec<_>>()
      .join("")
      .trim()
      .to_owned();

    ensure!(
      !output_string.is_empty(),
      "nix current-system process did not output utf-8 to stdout"
    );

    let output_string = output_string
      .strip_prefix("\"")
      .and_then(|o| o.strip_suffix("\""))
      .ok_or(miette!("nix current-system process output was not quoted"))?
      .to_owned();

    tracing::debug!(system = ?output_string, "got current system");

    Ok(CurrentSystem(output_string))
  }
}
