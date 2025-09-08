mod current_system;
mod path_info;

use std::{fmt, io::Read, str::FromStr};

use clap::Args;
use miette::{Context, IntoDiagnostic, bail, miette};
use models::{
  StorePath,
  dvf::{FileSize, StrictSlug},
};

use self::{current_system::CurrentSystem, path_info::PathInfo};
use crate::{Action, app_state::AppState, authenticate::AuthenticateCommand};

#[derive(Clone, Debug)]
pub(crate) struct Installable(String);

impl FromStr for Installable {
  type Err = !;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Installable(s.to_owned()))
  }
}

impl fmt::Display for Installable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

#[derive(Args, Debug)]
pub(crate) struct UploadCommand {
  /// The flake installable to upload.
  installable: Installable,
  /// The store to store the uploaded entries in.
  #[arg(long, short)]
  store:       String,
  /// The caches to upload to.
  #[arg(required = true, num_args = 1..)]
  caches:      Vec<String>,
}

impl Action for UploadCommand {
  type Error = miette::Report;
  type Output = ();

  async fn execute(
    self,
    app_state: &AppState,
  ) -> Result<Self::Output, Self::Error> {
    // get the pathinfo
    let pathinfo_result = PathInfo::calculate(&self.installable)
      .await
      .context(format!(
        "failed to get path-info for installable `{}`",
        self.installable
      ))?;
    let Some(pathinfo) = pathinfo_result.get().as_ref() else {
      bail!(
        "specified installable has not been built or fetched on this system: \
         `{installable}`",
        installable = self.installable
      );
    };

    let store_path = pathinfo_result.store_path();
    tracing::debug!(%store_path, "got path-info");
    let deriver_store_path: StorePath<String> = StorePath::from_absolute_path(
      pathinfo
        .deriver()
        .strip_suffix(".drv")
        .ok_or(miette!(
          "deriver path from `nix path-info` did not have \".drv\" suffix"
        ))?
        .as_bytes(),
    )
    .into_diagnostic()
    .context(
      "failed to parse deriver path from `nix path-info` as a store path",
    )?;

    let current_system = CurrentSystem::calculate()
      .await
      .context("failed to determine current system")?;

    let cache_list = self
      .caches
      .into_iter()
      .map(|c| (c.clone(), StrictSlug::new(c)))
      .inspect(|(o, n)| {
        if *o != n.clone().into_inner() {
          tracing::warn!("coercing cache name `{o}` into `{n}`")
        }
      })
      .map(|(_, s)| s.to_string())
      .collect::<Vec<_>>()
      .join(",");
    tracing::debug!(%cache_list, "using cache list");

    let target_store = StrictSlug::new(self.store.clone());
    if self.store != target_store.clone().into_inner() {
      tracing::warn!(
        "coercing store name `{original}` into `{new}`",
        original = self.store,
        new = target_store
      );
    }
    tracing::debug!(%target_store, "using target store");

    // authenticate with origin. session cookie gets saved in client.
    let _creds = (AuthenticateCommand {})
      .execute(app_state)
      .await
      .context("failed to authenticate")?;

    tracing::debug!(%store_path, "building NAR");
    let mut nar_reader = pathinfo_result
      .nar_encoder()
      .into_diagnostic()
      .context("failed to pack nix store path as a NAR")?;
    let mut buffer = Vec::new();
    nar_reader
      .read_to_end(&mut buffer)
      .into_diagnostic()
      .context("failed to read bytes from nar encoder")?;
    tracing::debug!("buffered {} of NAR", FileSize::new(buffer.len() as _));
    let nar_belt = belt::Belt::from_bytes(bytes::Bytes::from(buffer), None);

    let client = app_state.http_client();

    let url = format!("{}/upload", app_state.api_url_base());
    let req = client
      .post(url)
      .query(&[
        ("caches", cache_list),
        ("store_path", store_path.to_string()),
        ("target_store", target_store.to_string()),
        ("deriver_store_path", deriver_store_path.to_string()),
        ("deriver_system", current_system.to_string()),
      ])
      .body(reqwest::Body::wrap_stream(nar_belt));

    tracing::debug!("sending upload request");
    let resp = req
      .send()
      .await
      .into_diagnostic()
      .context("failed to send upload request")?;

    let text_resp = resp
      .text()
      .await
      .into_diagnostic()
      .context("failed to read response body")?;

    tracing::debug!(body = text_resp, "got upload response");

    let json_resp: serde_json::Value = serde_json::from_str(&text_resp)
      .into_diagnostic()
      .context("failed to deserialize response body as JSON")?;

    tracing::debug!(body = ?json_resp, "parsed upload response");

    Ok(())
  }
}
