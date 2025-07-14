#![feature(iterator_try_collect)]

//! The CLI entrypoint for Rambit.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::{Context, IntoDiagnostic, Result, bail};
use models::{
  StorePath, User,
  dvf::{self, EntityName, RecordId, StrictSlug},
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
    /// The caches to upload to.
    #[arg(long = "caches", value_parser = parse_cache_name, value_delimiter = ',', required = true)]
    cache_list:         Vec<EntityName>,
    /// The store path of the NAR.
    #[arg(long = "store-path", value_parser = parse_store_path)]
    store_path:         StorePath<String>,
    /// The store to store the entry data in.
    #[arg(long = "target-store", value_parser = parse_store_name)]
    target_store:       EntityName,
    /// The system that the NAR was derived on.
    #[arg(long = "deriver-system")]
    deriver_system:     String,
    /// The store path of the NAR's deriver.
    #[arg(long = "deriver-store-path", value_parser = parse_deriver_store_path)]
    deriver_store_path: StorePath<String>,
    /// The user uploading to the cache.
    #[arg(long = "user")]
    user_id:            RecordId<User>,
    /// The file to upload to the cache.
    #[arg(long = "nar")]
    nar_path:           PathBuf,
  },
}

fn parse_cache_name(input: &str) -> Result<EntityName, String> {
  if input.is_empty() {
    return Err("empty cache name found".to_owned());
  }
  match dvf::strict::strict_slugify(input) == input {
    true => Ok(EntityName::new(StrictSlug::new(input))),
    false => Err(format!("cache name is malformed: `{input}`")),
  }
}

fn parse_store_path(input: &str) -> Result<StorePath<String>, String> {
  if input.is_empty() {
    return Err("store path is empty".to_owned());
  }
  let input = StorePath::from_bytes(input.as_bytes())
    .map_err(|_| format!("store path is malformed: `{input}`"))?;
  Ok(input)
}

fn parse_deriver_store_path(input: &str) -> Result<StorePath<String>, String> {
  if input.is_empty() {
    return Err("deriver store path is empty".to_owned());
  }
  let input = StorePath::from_bytes(input.as_bytes())
    .map_err(|_| format!("deriver store path is malformed: `{input}`"))?;
  Ok(input)
}

fn parse_store_name(input: &str) -> Result<EntityName, String> {
  let sanitized = StrictSlug::new(input);
  if input != sanitized.as_ref() {
    return Err(format!(
      "invalid target store name \"{input}\" - try \"{sanitized}\""
    ));
  }
  Ok(EntityName::new(sanitized))
}

impl CliArgs {
  async fn execute(&self) -> Result<()> {
    match self.command {
      SubCommand::Upload {
        ref cache_list,
        ref store_path,
        ref target_store,
        ref deriver_system,
        ref deriver_store_path,
        user_id,
        ref nar_path,
      } => {
        let client = reqwest::Client::new();

        match tokio::fs::try_exists(nar_path).await {
          Ok(false) => {
            tracing::error!(?nar_path, "symlinks to input file are broken");
            bail!("symlinks to input file are broken: \"{nar_path:?}\"")
          }
          Err(_) => {
            tracing::error!(?nar_path, "input file does not exist");
            bail!("input file does not exist: \"{nar_path:?}\"")
          }
          _ => {}
        }
        tracing::debug!(?nar_path, "file exists");

        let file = tokio::fs::File::open(nar_path)
          .await
          .into_diagnostic()
          .context("failed to read file")?;
        tracing::debug!(?nar_path, "opened file");

        let data = belt::Belt::from_async_buf_read(BufReader::new(file), None);

        let cache_list =
          cache_list.iter().map(|c| c.to_string()).collect::<String>();
        let req = client
          .post(format!(
            "http://{host}:{port}/upload",
            host = self
              .host
              .as_ref()
              .cloned()
              .unwrap_or("localhost".to_string()),
            port = self.port.unwrap_or(3000),
          ))
          .header("x-user-id", user_id.to_string())
          .query(&[
            ("caches", cache_list),
            ("store_path", store_path.to_string()),
            ("target_store", target_store.to_string()),
            ("deriver_store_path", deriver_store_path.to_string()),
            ("deriver_system", deriver_system.to_string()),
          ])
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
