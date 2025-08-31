use clap::Args;
use miette::{Context, IntoDiagnostic, bail, miette};
use models::{User, dvf::RecordId};
use serde::Serialize;

use crate::{Action, AppState, SessionCreds};

#[derive(Serialize, Debug)]
struct AuthenticateRequestParams {
  email:    String,
  password: String,
}

#[derive(Args, Debug)]
pub(crate) struct AuthenticateCommand {}

impl Action for AuthenticateCommand {
  type Error = miette::Report;
  type Output = SessionCreds;

  async fn execute(
    self,
    app_state: &AppState,
  ) -> Result<Self::Output, Self::Error> {
    let (email, password) = match (&app_state.email, &app_state.password) {
      (Some(e), Some(p)) => (e, p),
      (None, None) => {
        bail!(
          help = "Authentication has not been configured. Supply email and \
                  password with arguments `--email` and `--password` or \
                  environment variables `RAMBIT_EMAIL` and `RAMBIT_PASSWORD`.",
          "missing authentication"
        );
      }
      (None, _) => {
        bail!(
          help = "Found password but no email address. Supply email with \
                  `--email` or environment variable `RAMBIT_EMAIL`.",
          "missing email"
        );
      }
      (_, None) => {
        bail!(
          help = "Found email address but no password. Supply password with \
                  argument `--password` or environment variable \
                  `RAMBIT_PASSWORD`.",
          "missing password"
        );
      }
    };

    tracing::debug!("authenticating as \"{email}\"");

    let client = app_state.http_client();

    let params = AuthenticateRequestParams {
      email:    email.clone().into_inner(),
      password: password.into(),
    };
    let url = format!("{}/authenticate", app_state.api_url_base());

    let req = client
      .post(url)
      .json(&params)
      .build()
      .into_diagnostic()
      .context("failed to build authenticate request")?;

    let resp = client
      .execute(req)
      .await
      .into_diagnostic()
      .context("failed to send authenticate request")?
      .error_for_status()
      .into_diagnostic()
      .context("authenticate request returned error")?;

    let session_cookie = resp
      .cookies()
      .find(|c| c.name() == "id")
      .map(|c| c.value().to_owned())
      .ok_or(miette!("failed to find session cookie"))?;

    let user_id = resp
      .json::<RecordId<User>>()
      .await
      .into_diagnostic()
      .context("failed to read authenticate response")?;

    tracing::debug!("authenticated successfully");

    let session = SessionCreds {
      user_id,
      session_cookie,
    };

    {
      *app_state.session.write().await = Some(session.clone());
    }

    Ok(session)
  }
}
