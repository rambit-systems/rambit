use std::{fmt::Debug, hash::Hash, time::Duration};

use leptos_fetch::QueryScope;

#[cfg(not(feature = "hydrate"))]
pub fn refetch_while_focused<
  K: Clone + Hash + PartialEq + Debug + 'static,
  KF: Fn() -> K + Copy + 'static,
  R: Clone + Debug + 'static,
>(
  _key_fn: KF,
  _query_scope: QueryScope<K, R>,
  _period: Duration,
) {
}

#[cfg(feature = "hydrate")]
pub fn refetch_while_focused<
  K: Clone + Hash + PartialEq + Debug + 'static,
  KF: Fn() -> K + Copy + 'static,
  R: Clone + Debug + 'static,
>(
  key_fn: KF,
  query_scope: QueryScope<K, R>,
  period: Duration,
) {
  use leptos::prelude::*;
  use leptos_fetch::QueryClient;
  use leptos_use::{use_interval_fn, use_window_focus};

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
