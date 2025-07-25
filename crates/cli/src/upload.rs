use std::path::Path;

use miette::{Context, IntoDiagnostic, bail};
use models::{
  StorePath, User,
  dvf::{EntityName, RecordId},
};
use tokio::io::BufReader;

pub async fn upload(
  host: &Option<String>,
  port: &Option<u16>,
  cache_list: &[EntityName],
  store_path: &StorePath<String>,
  target_store: &EntityName,
  deriver_system: &str,
  deriver_store_path: &StorePath<String>,
  user_id: RecordId<User>,
  nar_path: &Path,
) -> miette::Result<()> {
  let client = reqwest::Client::new();

  match tokio::fs::try_exists(nar_path).await {
    Ok(false) => {
      tracing::error!(?nar_path, "symlinks to input NAR are broken");
      bail!("symlinks to input NAR are broken: \"{nar_path:?}\"")
    }
    Err(_) => {
      tracing::error!(?nar_path, "input NAR does not exist");
      bail!("input NAR does not exist: \"{nar_path:?}\"")
    }
    _ => {}
  }
  tracing::debug!(?nar_path, "NAR exists");

  let file = tokio::fs::File::open(nar_path)
    .await
    .into_diagnostic()
    .context("failed to read NAR")?;
  tracing::debug!(?nar_path, "opened NAR");

  let data = belt::Belt::from_async_buf_read(BufReader::new(file), None);

  let cache_list = cache_list.iter().map(|c| c.to_string()).collect::<String>();
  let req = client
    .post(format!(
      "http://{host}:{port}/upload",
      host = host.as_ref().cloned().unwrap_or("localhost".to_string()),
      port = port.unwrap_or(3000),
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

  tracing::debug!(?resp, "sent request");
  tracing::debug!("response body: {}", resp.text().await.unwrap());

  Ok(())
}
