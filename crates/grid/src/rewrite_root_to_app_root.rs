use app::APP_PREFIX;
use axum::{extract::Request, middleware::Next, response::IntoResponse};
use http::Uri;

/// Rewrites the URI to point to `app::APP_PREFIX` instead of the root.
pub(crate) async fn rewrite_root_to_app_root(
  mut req: Request,
  next: Next,
) -> impl IntoResponse {
  if req.uri().path() == "/" {
    let original = req.uri();

    // the new path and query string
    let new_pq = match original.query() {
      Some(q) => format!("{APP_PREFIX}?{q}"),
      None => APP_PREFIX.to_string(),
    };

    // rebuild and replace the URI
    let mut parts = original.clone().into_parts();
    parts.path_and_query = Some(new_pq.parse().unwrap());
    *req.uri_mut() = Uri::from_parts(parts).unwrap();
    tracing::debug!("rewrote URI from root to app root ({APP_PREFIX:?})");
  }
  next.run(req).await
}
