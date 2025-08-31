//! The CLI entrypoint for Rambit.

#![feature(exit_status_error)]
#![feature(never_type)]

mod app_state;
mod authenticate;
mod upload;

use clap::{Parser, Subcommand};
use miette::{Context, Result};
use models::{User, dvf::RecordId};
use tracing_subscriber::{
  EnvFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

use self::{
  app_state::AppState, authenticate::AuthenticateCommand, upload::UploadCommand,
};

#[expect(dead_code)]
#[derive(Debug, Clone)]
struct SessionCreds {
  user_id:        RecordId<User>,
  session_cookie: String,
}

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
  /// The Rambit host.
  #[arg(long, short)]
  pub addr:       Option<String>,
  /// Whether to force HTTP-only.
  #[arg(long)]
  pub http_only:  Option<bool>,
  /// The user's email.
  #[arg(long, short)]
  pub email:      Option<String>,
  /// The user's password.
  #[arg(long, short)]
  pub password:   Option<String>,
  /// The given subcommand.
  #[command(subcommand)]
  pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
  Authenticate(AuthenticateCommand),
  Upload(UploadCommand),
}

impl SubCommand {
  pub(crate) async fn execute(self, app_state: &AppState) -> Result<()> {
    match self {
      SubCommand::Authenticate(authenticate) => {
        authenticate
          .execute(app_state)
          .await
          .context("failed to execute `authenticate` subcommand")?;
      }
      SubCommand::Upload(upload) => {
        upload
          .execute(app_state)
          .await
          .context("failed to execute `upload` subcommand")?;
      }
    }

    Ok(())
  }
}

pub(crate) trait Action {
  type Error;
  type Output;

  async fn execute(
    self,
    app_state: &AppState,
  ) -> Result<Self::Output, Self::Error>;
}

#[tokio::main]
async fn main() -> Result<()> {
  let default_directive = "info,cli=debug"
    .parse()
    .expect("failed to parse logging directive");
  let env_filter = EnvFilter::builder()
    .parse_lossy(std::env::var("RUST_LOG").ok().unwrap_or(default_directive));
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false),
    )
    .with(env_filter)
    .init();

  tracing::info!("Welcome to the Rambit CLI :)");

  let args = CliArgs::parse();
  let app_state = AppState::from_args(&args).await?;

  args
    .subcommand
    .execute(&app_state)
    .await
    .context("failed to execute subcommand")?;

  Ok(())
}
