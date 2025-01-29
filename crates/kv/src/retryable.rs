use std::fmt::Debug;

use hex::retryable::Retryable;

use crate::*;

#[async_trait::async_trait]
impl<KV: KvTransactional, E: Debug + Send + Sync + 'static> KvTransactional
  for Retryable<KV, E>
{
  async fn begin_optimistic_transaction(&self) -> KvResult<DynTransaction> {
    let result = self.inner();
    match result {
      Ok(kv) => kv.begin_optimistic_transaction().await,
      Err(err) => Err(KvError::PlatformError(miette::miette!(
        "KV store is statefully errored: {err:?}"
      ))),
    }
  }
  async fn begin_pessimistic_transaction(&self) -> KvResult<DynTransaction> {
    let result = self.inner();
    match result {
      Ok(kv) => kv.begin_pessimistic_transaction().await,
      Err(err) => Err(KvError::PlatformError(miette::miette!(
        "KV store is statefully errored: {err:?}"
      ))),
    }
  }
}
