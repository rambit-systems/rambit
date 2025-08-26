use std::{fmt::Debug, hash::Hash, time::Duration};

use leptos::prelude::*;
use leptos_fetch::{QueryClient, QueryScope};
use leptos_use::{use_interval_fn, use_window_focus};
use serde::{de::DeserializeOwned, Serialize};

pub fn refetch_while_focused<
  K: Clone + Hash + PartialEq + Debug + Send + Sync + 'static,
  KF: Fn() -> K + Copy + Send + Sync + 'static,
  R: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
>(
  key_fn: KF,
  query_scope: QueryScope<K, R>,
  period: Duration,
) {
  let query_client = expect_context::<QueryClient>();
  let period_ms = Signal::stored(period.as_millis() as u64);

  let invalidate = {
    let query_scope = query_scope.clone();
    move || {
      query_client.invalidate_query(query_scope.clone(), key_fn());
    }
  };

  let focused = use_window_focus();

  let _ = use_interval_fn(
    move || {
      if focused() {
        invalidate()
      }
    },
    period_ms,
  );
}
