use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::Result;
use models::{
  StorePath, User,
  dvf::{self, EntityName, RecordId, StrictSlug},
};

#[derive(Parser, Debug)]
pub struct CliArgs {
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
pub enum SubCommand {
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
  pub async fn execute(&self) -> Result<()> {
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
        crate::upload::upload(
          &self.host,
          &self.port,
          cache_list,
          store_path,
          target_store,
          deriver_system,
          deriver_store_path,
          user_id,
          nar_path,
        )
        .await
      }
    }
  }
}
