use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use axum::http::{HeaderValue, Request, Response};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct CacheOnSuccessLayer;

impl CacheOnSuccessLayer {
  pub fn new() -> Self { Self }
}

impl<S> Layer<S> for CacheOnSuccessLayer {
  type Service = CacheOnSuccessService<S>;

  fn layer(&self, inner: S) -> Self::Service { CacheOnSuccessService { inner } }
}

#[derive(Clone)]
pub struct CacheOnSuccessService<S> {
  inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for CacheOnSuccessService<S>
where
  S: Service<Request<ReqBody>, Response = Response<ResBody>>,
  S::Future: Send + 'static,
{
  type Error = S::Error;
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
  type Response = S::Response;

  fn poll_ready(
    &mut self,
    cx: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
    let future = self.inner.call(req);

    Box::pin(async move {
      let mut response = future.await?;

      // Check if the response status is successful (2xx range)
      if response.status().is_success() {
        // Set Cache-Control header
        let cache_control_value =
          HeaderValue::from_static("max-age=31536000, immutable");
        response
          .headers_mut()
          .insert("cache-control", cache_control_value);
      }

      Ok(response)
    })
  }
}

// Usage example
#[cfg(test)]
mod tests {
  use axum::http::{Request, Response, StatusCode};
  use tower::{ServiceBuilder, ServiceExt, service_fn};

  use super::*;

  #[tokio::test]
  async fn test_cache_control_middleware() {
    // Create a simple service that returns a 200 OK response
    let service = service_fn(|_req: Request<()>| async {
      Ok::<_, std::convert::Infallible>(
        Response::builder().status(StatusCode::OK).body(()).unwrap(),
      )
    });

    // Wrap with our middleware
    let mut service = ServiceBuilder::new()
      .layer(CacheOnSuccessLayer::new())
      .service(service);

    // Make a request
    let request = Request::builder().body(()).unwrap();
    let response = service.ready().await.unwrap().call(request).await.unwrap();

    // Check that Cache-Control header is set for successful response
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
      response.headers().get("cache-control").unwrap(),
      "max-age=31536000, immutable"
    );
  }

  #[tokio::test]
  async fn test_no_cache_control_on_error() {
    // Create a service that returns a 404 Not Found response
    let service = service_fn(|_req: Request<()>| async {
      Ok::<_, std::convert::Infallible>(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body(())
          .unwrap(),
      )
    });

    // Wrap with our middleware
    let mut service = ServiceBuilder::new()
      .layer(CacheOnSuccessLayer::new())
      .service(service);

    // Make a request
    let request = Request::builder().body(()).unwrap();
    let response = service.ready().await.unwrap().call(request).await.unwrap();

    // Check that Cache-Control header is NOT set for error response
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert!(response.headers().get("cache-control").is_none());
  }
}
