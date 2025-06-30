//! The CLI entrypoint for Rambit.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::{Context, IntoDiagnostic, Result, bail};
use models::{
  User,
  dvf::{LaxSlug, RecordId, StrictSlug},
};
use tokio::io::BufReader;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

#[derive(Parser, Debug)]
struct CliArgs {
  #[command(subcommand)]
  pub command: SubCommand,
  /// The Rambit host to connect to.
  #[arg(long)]
  pub host:    Option<String>,
  /// The port of the Rambit host to connect to.
  #[arg(long)]
  pub port:    Option<u16>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
  Upload {
    /// The name of the cache to upload to.
    #[arg(long = "cache", value_parser = parse_cache_name)]
    cache_name:   StrictSlug,
    /// The desired final path of the entry.
    #[arg(long = "entry-path", value_parser = parse_desired_path)]
    desired_path: LaxSlug,
    /// The store to store the entry data in.
    #[arg(long = "store", value_parser = parse_store_name)]
    target_store: Option<StrictSlug>,
    /// The user uploading to the cache.
    #[arg(long = "user")]
    user_id:      RecordId<User>,
    /// The file to upload to the cache.
    #[arg(long = "file")]
    file_path:    PathBuf,
  },
}

fn parse_cache_name(input: &str) -> Result<StrictSlug, String> {
  let sanitized = StrictSlug::new(input);
  if input != sanitized.as_ref() {
    return Err(format!(
      "invalid cache name \"{input}\" - try \"{sanitized}\""
    ));
  }
  Ok(sanitized)
}

fn parse_desired_path(input: &str) -> Result<LaxSlug, String> {
  let sanitized = LaxSlug::new(input);
  if input != sanitized.as_ref() {
    return Err(format!(
      "invalid desired path \"{input}\" - try \"{sanitized}\""
    ));
  }
  Ok(sanitized)
}

fn parse_store_name(input: &str) -> Result<StrictSlug, String> {
  let sanitized = StrictSlug::new(input);
  if input != sanitized.as_ref() {
    return Err(format!(
      "invalid store name \"{input}\" - try \"{sanitized}\""
    ));
  }
  Ok(sanitized)
}

impl CliArgs {
  async fn execute(&self) -> Result<()> {
    match self.command {
      SubCommand::Upload {
        ref cache_name,
        ref desired_path,
        ref target_store,
        user_id,
        ref file_path,
      } => {
        let client = reqwest::Client::new();

        match tokio::fs::try_exists(file_path).await {
          Ok(false) => {
            tracing::error!(?file_path, "symlinks to input file are broken");
            bail!("symlinks to input file are broken: \"{file_path:?}\"")
          }
          Err(_) => {
            tracing::error!(?file_path, "input file does not exist");
            bail!("input file does not exist: \"{file_path:?}\"")
          }
          _ => {}
        }
        tracing::debug!(?file_path, "file exists");

        let file = tokio::fs::File::open(file_path)
          .await
          .into_diagnostic()
          .context("failed to read file")?;
        tracing::debug!(?file_path, "opened file");

        let data = belt::Belt::from_async_buf_read(BufReader::new(file), None);

        let req = client
          .post(format!(
            "http://{host}:{port}/upload/{cache_name}/{desired_path}/{store}",
            host = self
              .host
              .as_ref()
              .cloned()
              .unwrap_or("localhost".to_string()),
            port = self.port.unwrap_or(3000),
            store = target_store
              .as_ref()
              .map(|s| s.to_string())
              .unwrap_or_default()
          ))
          .header("user_id", user_id.to_string())
          .body(reqwest::Body::wrap_stream(data));

        let resp = req
          .send()
          .await
          .into_diagnostic()
          .context("failed to send request")?;

        tracing::info!(?resp, "sent request");
        tracing::info!("response body: {}", resp.text().await.unwrap());

        Ok(())
      }
    }
  }
}

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
