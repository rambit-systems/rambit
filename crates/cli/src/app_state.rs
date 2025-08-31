use std::net::SocketAddr;

use miette::{Context, IntoDiagnostic, Result};
use models::dvf::EmailAddress;
use reqwest::Client;
use tokio::sync::RwLock;

use crate::{CliArgs, SessionCreds};

#[derive(Debug)]
pub(crate) struct AppState {
  pub rambit_addr: String,
  pub http_only:   bool,
  pub email:       Option<EmailAddress>,
  pub password:    Option<String>,
  pub session:     RwLock<Option<SessionCreds>>,
  pub http_client: reqwest::Client,
}

impl AppState {
  pub(crate) async fn from_args(args: &CliArgs) -> Result<Self> {
    let rambit_addr = args
      .addr
      .clone()
      .or_else(|| std::env::var("RAMBIT_ADDR").ok())
      .unwrap_or("rambit.app:443".to_owned());
    // let rambit_addr = tokio::net::lookup_host(&rambit_addr)
    //   .await
    //   .into_diagnostic()
    //   .context("failed to lookup host")?
    //   .nth(0)
    //   .expect("address iterator held no elements");

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
      http_client: Client::builder()
        .cookie_store(true)
        .build()
        .expect("failed to build http client"),
    })
  }

  pub(crate) fn api_url_base(&self) -> String {
    format!(
      "{protocol}://{addr}/api/v1",
      protocol = match self.http_only {
        true => "http",
        false => "https",
      },
      addr = self.rambit_addr
    )
  }

  pub(crate) fn http_client(&self) -> Client { self.http_client.clone() }
}
