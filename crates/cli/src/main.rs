#![feature(iterator_try_collect)]

//! The CLI entrypoint for Rambit.

mod args;

use clap::Parser;
use miette::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

use self::args::CliArgs;

#[tokio::main]
async fn main() -> Result<()> {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
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
  args.execute().await?;

  Ok(())
}
