use std::net::SocketAddr;

use miette::{Context, IntoDiagnostic, Result};
use models::dvf::EmailAddress;
use tokio::sync::RwLock;

use crate::{CliArgs, SessionCreds};

#[derive(Debug)]
pub(crate) struct AppState {
  pub rambit_addr: SocketAddr,
  pub http_only:   bool,
  pub email:       Option<EmailAddress>,
  pub password:    Option<String>,
  pub session:     RwLock<Option<SessionCreds>>,
}

impl AppState {
  pub(crate) async fn from_args(args: &CliArgs) -> Result<Self> {
    let rambit_addr = args
      .addr
      .clone()
      .or_else(|| std::env::var("RAMBIT_ADDR").ok())
      .unwrap_or("rambit.app:443".to_owned());
    let rambit_addr = tokio::net::lookup_host(&rambit_addr)
      .await
      .into_diagnostic()
      .context("failed to lookup host")?
      .nth(0)
      .expect("address iterator held no elements");

    let email = args
      .email
      .clone()
      .or_else(|| std::env::var("RAMBIT_EMAIL").ok())
      .map(|v| {
        EmailAddress::try_new(v)
          .into_diagnostic()
          .context("failed to parse email address")
      })
      .transpose()?;

    let password = args
      .password
      .clone()
      .or_else(|| std::env::var("RAMBIT_PASSWORD").ok());

    let http_only = args
      .http_only
      .or_else(|| {
        std::env::var("RAMBIT_HTTP_ONLY")
          .ok()
          .map(|s| matches!(s.as_str(), "true" | "TRUE" | "1"))
      })
      .unwrap_or(false);

    Ok(AppState {
      rambit_addr,
      http_only,
      email,
      password,
      session: RwLock::default(),
    })
  }
}
