use std::{fmt::Debug, hash::Hash, time::Duration};

use leptos_fetch::QueryScope;

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
