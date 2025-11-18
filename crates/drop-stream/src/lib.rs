//! Provides an async-drop stream wrapper.
use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use futures::Stream;

/// A stream wrapper that **spawns an async callback when dropped**.
///
/// The wrapper holds:
/// - an inner stream,
/// - a future that is spawned via `tokio::spawn` upon drop.
///
/// ## Notes
/// - The callback future must be `Send + 'static`.
/// - Requires a running Tokio runtime at the moment of drop.
/// - This version **does not use `pin-project`**.
pub struct StreamWithDropCallback<S> {
  inner:    S,
  callback: Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
}

impl<S> StreamWithDropCallback<S> {
  /// Wraps a stream and registers an async callback to be spawned on drop.
  pub fn new<F>(inner: S, fut: F) -> Self
  where
    F: Future<Output = ()> + Send + 'static,
  {
    Self {
      inner,
      callback: Some(Box::pin(fut)),
    }
  }
}

impl<S: Stream> Stream for StreamWithDropCallback<S> {
  type Item = S::Item;

  /// Polls the wrapped stream.
  ///
  /// Since we are not using pin-project, we manually project `inner`
  /// using `Pin::new_unchecked`, which is safe because `inner` is never
  /// moved after being pinned (the struct is pinned as a whole).
  fn poll_next(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    // SAFETY:
    // - We never move `inner` after pinning the outer struct.
    // - We do not move `inner` out from behind &mut self.inner.
    // - Therefore `Pin::new_unchecked(&mut self.inner)` is safe.
    let inner =
      unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().inner) };
    inner.poll_next(cx)
  }
}

impl<S> Drop for StreamWithDropCallback<S> {
  /// Spawns the async callback (if any) via `tokio::spawn`.
  fn drop(&mut self) {
    if let Some(fut) = self.callback.take() {
      tokio::spawn(fut);
    }
  }
}

/// Extension trait adding `.on_drop_async(future)` to any stream.
///
/// ## Example
/// ```rust,no_run
/// use drop_stream::StreamDropCallbackExt;
/// use futures::stream::StreamExt;
///
/// let s = futures::stream::iter([1, 2, 3]).on_drop_async(async {
///   println!("dropped!");
/// });
/// ```
pub trait StreamDropCallbackExt: Stream + Sized {
  /// Returns a wrapper that spawns the async future when dropped.
  fn on_drop_async<F>(self, fut: F) -> StreamWithDropCallback<Self>
  where
    F: Future<Output = ()> + Send + 'static,
  {
    StreamWithDropCallback::new(self, fut)
  }
}

impl<T: Stream + Sized> StreamDropCallbackExt for T {}

#[cfg(test)]
mod tests {
  use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  };

  use futures::stream::{self, StreamExt};

  use super::*;

  #[tokio::test]
  async fn drop_callback_runs_on_drop() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter2 = counter.clone();

    {
      let s = stream::iter([1, 2, 3]).on_drop_async(async move {
        counter2.fetch_add(1, Ordering::SeqCst);
      });

      // Consume partially; we don't need the whole stream.
      futures::pin_mut!(s);
      assert_eq!(s.next().await, Some(1));
    } // <-- drop happens here, callback should fire

    tokio::task::yield_now().await; // give the spawned task a chance to run

    assert_eq!(
      counter.load(Ordering::SeqCst),
      1,
      "drop callback should have incremented counter"
    );
  }

  #[tokio::test]
  async fn drop_callback_does_not_run_early() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter2 = counter.clone();

    {
      let s = stream::iter([1, 2, 3]).on_drop_async(async move {
        counter2.fetch_add(1, Ordering::SeqCst);
      });

      futures::pin_mut!(s);

      // Poll the stream but do not drop yet.
      assert_eq!(s.next().await, Some(1));
      assert_eq!(s.next().await, Some(2));

      // Callback should *not* have executed yet.
      assert_eq!(
        counter.load(Ordering::SeqCst),
        0,
        "callback should not run before drop"
      );
    } // dropped

    tokio::task::yield_now().await; // allow callback to run

    assert_eq!(
      counter.load(Ordering::SeqCst),
      1,
      "callback should run exactly once at drop"
    );
  }
}
