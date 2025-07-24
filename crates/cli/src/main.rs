#![feature(iterator_try_collect)]

//! The CLI entrypoint for Rambit.

mod args;

use clap::Parser;
use miette::Result;
use tracing_subscriber::{EnvFilter, prelude::*};

use self::args::CliArgs;

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
  args.execute().await?;

  Ok(())
}
