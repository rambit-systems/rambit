use clap::Parser;
/// The Rambit app CLI.
#[derive(Parser)]
pub struct CliArgs {
  /// Whether to run database migrations.
  #[arg(long)]
  pub migrate:           bool,
  #[arg(long, default_value_t = false)]
  pub no_secure_cookies: bool,
  #[arg(long, default_value_t = 3000)]
  pub port:              u16,
  #[arg(long, default_value = "[::]")]
  pub host:              String,
}
