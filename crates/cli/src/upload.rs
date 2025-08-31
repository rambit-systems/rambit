mod path_info;

use std::{fmt, str::FromStr};

use clap::Args;
use miette::{Context, bail};

use self::path_info::PathInfo;
use crate::{Action, app_state::AppState};

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
  installable: Installable,
}

impl Action for UploadCommand {
  type Error = miette::Report;
  type Output = ();

  async fn execute(
    self,
    _app_state: &AppState,
  ) -> Result<Self::Output, Self::Error> {
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

    Ok(())
  }
}
