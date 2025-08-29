use std::collections::HashSet;

use miette::{Context, IntoDiagnostic, Result, bail, ensure};
use models::{CAHash, StorePath, dvf::FileSize};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

use super::Installable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PathInfo {
  #[serde(alias = "ca")]
  ca_hash:    Option<CAHash>,
  // deriver:    String,
  #[serde(alias = "narHash")]
  nar_hash:   String,
  #[serde(alias = "narSize")]
  nar_size:   FileSize,
  references: HashSet<StorePath<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PathInfoResult {
  store_path: StorePath<String>,
  data:       Option<PathInfo>,
}

impl PathInfo {
  pub(crate) async fn get(installable: &Installable) -> Result<PathInfoResult> {
    tracing::debug!(%installable, "getting path-info");

    let mut command = Command::new("nix");

    command.env("NIX_PATH", "");

    command.args(["path-info", "--json"]);
    command.args(["--extra-experimental-features", "nix-command flakes"]);
    command.args(["--option", "warn-dirty", "false"]);
    command.arg(installable.to_string());

    let output = command
      .output()
      .await
      .into_diagnostic()
      .context("failed to spawn `nix path-info` process")?;

    let root = serde_json::from_slice::<serde_json::Value>(&output.stdout)
      .into_diagnostic()
      .context("failed to deserialize JSON from `nix path-info` output")?;

    ensure!(
      root.is_object(),
      "`nix path-info` JSON output is not an object"
    );
    let root_object = root.as_object().unwrap();
    ensure!(
      root_object.len() == 1,
      "`nix path-info` JSON output does not have 1 key"
    );

    let store_path_string = root_object.keys().nth(0).unwrap();
    let store_path =
      StorePath::from_absolute_path(store_path_string.as_bytes())
        .into_diagnostic()
        .context(
          "failed to deserialize store path from `nix path-info` JSON output",
        )?;

    let data = match root_object.get(store_path_string).unwrap() {
      serde_json::Value::Null => None,
      serde_json::Value::Object(map) => Some(
        PathInfo::deserialize(&serde_json::Value::Object(map.to_owned()))
          .into_diagnostic()
          .context(
            "failed to deserialize path data from `nix path-info` JSON output",
          )?,
      ),
      v => bail!(
        "got unexpected data from store-path key in `nix path-info` JSON \
         output: {v}"
      ),
    };

    Ok(PathInfoResult { store_path, data })
  }
}
