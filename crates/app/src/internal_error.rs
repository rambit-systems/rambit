use axum::response::IntoResponse;
use maud::html;

/// A page-wide internal error rejection for large-scale failures.
pub struct InternalErrorRejection;

impl From<miette::Report> for InternalErrorRejection {
  fn from(error: miette::Report) -> Self {
    tracing::error!("internal error reported: {error:#?}");
    InternalErrorRejection
  }
}

impl IntoResponse for InternalErrorRejection {
  fn into_response(self) -> axum::response::Response {
    let (_ctx, resp) = columbo::new();

    let page = html! {
      h1 { "An internal error occurred." }
    };

    resp.into_stream(page).into_response()
  }
}
