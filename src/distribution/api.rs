use hyper::{Body, Request, Response};
use tower::Service;

use crate::{BoxError, Result};

use super::client::Client;

/// To check whether or not the registry implements this specification, perform a GET request to the following endpoint: /v2/
///
/// If the response is 200 OK, then the registry implements this specification.
///
/// This endpoint MAY be used for authentication/authorization purposes, but this is out of the purview of this specification.
#[derive(Clone)]
pub struct SupportAPI<S> {
    client: Client<S>,
}

impl<S> SupportAPI<S> {
    /// Create a [`SupportAPI`].
    pub fn new(client: Client<S>) -> Self {
        Self { client }
    }
}

impl<S> SupportAPI<S>
where
    S: Clone,
    S: Service<Request<Body>, Response = Response<Body>>,
    S::Error: Into<BoxError>,
{
    /// Check whether or not the registry implements this specification.
    pub async fn check(&self, request: SupportRequest) -> Result<SupportResponse> {
        let request = request.request().unwrap();

        let mut client = self.client.clone();

        let response = client.send(request).await?;

        let (parts, _) = response.into_parts();

        Ok(SupportResponse { parts })
    }
}

///
pub struct SupportRequest {
    inner: hyper::http::request::Builder,
}

impl SupportRequest {
    /// Create a [`SupportRequest`].
    pub fn new() -> Self {
        Self {
            inner: Request::builder(),
        }
    }

    ///
    pub fn base_url(self, value: &str) -> Self {
        Self {
            inner: self.inner.uri(format!("{value}/v2/")),
        }
    }

    fn request(self) -> Result<Request<Body>, hyper::http::Error> {
        self.inner.body(hyper::Body::empty())
    }
}

///
pub struct SupportResponse {
    parts: hyper::http::response::Parts,
}

impl SupportResponse {
    ///
    pub fn status(&self) -> hyper::http::StatusCode {
        self.parts.status
    }
}
