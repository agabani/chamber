use std::{future::Future, pin::Pin};

use hyper::{Body, Request, Response};
use tower::{Layer, Service};

/// Layer to check whether or not the registry implements distribution specification.
#[allow(clippy::module_name_repetitions)]
pub struct SupportLayer;

impl<S> Layer<S> for SupportLayer {
    type Service = SupportService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SupportService { inner }
    }
}

/// Service to check whether or not the registry implements distribution specification.
#[allow(clippy::module_name_repetitions)]
pub struct SupportRequest {
    inner: hyper::http::request::Builder,
}

impl SupportRequest {
    /// Create a [`SupportRequest`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Request::builder(),
        }
    }

    /// Set the URI for this request.
    #[must_use]
    pub fn base_uri(self, value: &str) -> Self {
        Self {
            inner: self.inner.uri(format!("{value}/v2/")),
        }
    }

    fn request(self) -> Result<Request<Body>, hyper::http::Error> {
        self.inner.body(hyper::Body::empty())
    }
}

impl Default for SupportRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Request to check whether or not the registry implements distribution specification.
#[allow(clippy::module_name_repetitions)]
pub struct SupportResponse {
    parts: hyper::http::response::Parts,
}

impl SupportResponse {
    /// The response's status.
    #[must_use]
    pub fn status(&self) -> hyper::http::StatusCode {
        self.parts.status
    }
}

/// Response to check whether or not the registry implements distribution specification.
#[allow(clippy::module_name_repetitions)]
pub struct SupportService<T> {
    inner: T,
}

impl<S> Service<SupportRequest> for SupportService<S>
where
    S: Service<Request<Body>, Response = Response<Body>>,
    S::Error: Into<crate::Error>,
    S::Future: 'static,
{
    type Response = SupportResponse;

    type Error = crate::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: SupportRequest) -> Self::Future {
        let request = req.request();

        let request = match request {
            Ok(request) => request,
            Err(err) => {
                todo!("{err}");
            }
        };

        let fut = self.inner.call(request);

        let f = async move {
            let result = fut.await;

            result
                .map(|response| {
                    let (parts, _) = response.into_parts();
                    SupportResponse { parts }
                })
                .map_err(Into::into)
        };

        Box::pin(f)
    }
}

#[cfg(test)]
mod tests {
    use hyper::{http::StatusCode, Body, Method, Request, Response};
    use tower::{Layer, Service, ServiceExt};
    use tower_test::mock;

    use super::{SupportLayer, SupportRequest};

    #[tokio::test]
    async fn calls() {
        // Arrange
        let (service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

        let spawned = tokio::spawn(async move {
            // Arrange
            let (request, send) = handle.next_request().await.expect("service not called");

            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap();

            send.send_response(response);

            // Assert
            assert_eq!(request.method(), Method::GET);
            assert_eq!(request.uri(), "http://registry.example.com/v2/");
        });

        let mut support = SupportLayer.layer(service);

        let request = SupportRequest::new().base_uri("http://registry.example.com");

        // Act
        let response = support.ready().await.unwrap().call(request).await.unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        spawned.await.unwrap();
    }
}
