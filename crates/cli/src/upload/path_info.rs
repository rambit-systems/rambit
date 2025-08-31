use std::collections::HashSet;

use miette::{Result, miette};
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

impl PathInfoResult {
  pub(crate) fn get(&self) -> &Option<PathInfo> { &self.data }

  pub(crate) fn store_path(&self) -> &StorePath<String> { &self.store_path }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub(crate) enum PathInfoError {
  #[error("failed to spawn `nix path-info` process: {0}")]
  Process(std::io::Error),
  #[error("failed to deserialize JSON from `nix path-info` output: {0}")]
  JsonParse(serde_json::Error),
  #[error("`nix path-info` JSON output form is unexpected: {0}")]
  JsonValidation(miette::Report),
  #[error("failed to deserialize store path: {0}")]
  StorePathDeserialization(models::nix_compat::store_path::Error),
  #[error("failed to deserialize path info: {0}")]
  PathInfoDeserialization(serde_json::Error),
}

impl PathInfo {
  pub(crate) async fn calculate(
    installable: &Installable,
  ) -> Result<PathInfoResult, PathInfoError> {
    tracing::debug!(%installable, "getting path-info");

    let mut command = Command::new("nix");

    command.env("NIX_PATH", "");

    command.args(["path-info", "--json"]);
    command.args(["--extra-experimental-features", "nix-command flakes"]);
    command.args(["--option", "warn-dirty", "false"]);
    command.arg(installable.to_string());

    let output = command.output().await.map_err(PathInfoError::Process)?;

    let root = serde_json::from_slice::<serde_json::Value>(&output.stdout)
      .map_err(PathInfoError::JsonParse)?;

    if !root.is_object() {
      Err(PathInfoError::JsonValidation(miette!(
        "`nix path-info` JSON output is not an object"
      )))?;
    }
    let root_object = root.as_object().unwrap();
    if root_object.len() != 1 {
      Err(PathInfoError::JsonValidation(miette!(
        "`nix path-info` JSON output does not have 1 key"
      )))?;
    }

    let store_path_string = root_object.keys().nth(0).unwrap();
    let store_path =
      StorePath::from_absolute_path(store_path_string.as_bytes())
        .map_err(PathInfoError::StorePathDeserialization)?;

    let data = match root_object.get(store_path_string).unwrap() {
      serde_json::Value::Null => None,
      serde_json::Value::Object(map) => Some(
        PathInfo::deserialize(&serde_json::Value::Object(map.to_owned()))
          .map_err(PathInfoError::PathInfoDeserialization)?,
      ),
      v => Err(PathInfoError::JsonValidation(miette!(
        "got unexpected data from store-path key in `nix path-info` JSON \
         output: {v}"
      )))?,
    };

    Ok(PathInfoResult { store_path, data })
  }
}
