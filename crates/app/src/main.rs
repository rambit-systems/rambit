//! The server-side entrypoint for Rambit.

mod app_state;

use clap::Parser;
use miette::{Context, Result};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

use self::app_state::AppState;

/// The Rambit app CLI.
#[derive(Parser)]
struct CliArgs {
  /// Whether to run database migrations.
  #[arg(short, long)]
  migrate: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(env_filter)
    .init();

  let args = CliArgs::parse();

  tracing::info!("starting app server");

  let app_state = AppState::build()
    .await
    .context("failed to build app state")?;

  if args.migrate {
    app_state
      .prime_domain
      .migrate_test_data(false)
      .await
      .context("failed to migrate test data")?;
  }

  Ok(())
}
