use hyper::{Body, Request, Response};
use tower::{Service, ServiceExt};

use crate::{Error, Result};

/// Client for connecting with a container registry.
#[derive(Clone)]
pub struct Client<S> {
    inner: S,
}

impl<S> Client<S> {
    /// Create a [`Client`].
    pub fn new(service: S) -> Self {
        Self { inner: service }
    }
}

impl<S> Client<S>
where
    S: Service<Request<Body>, Response = Response<Body>, Error = hyper::Error>,
{
    /// Send a raw HTTP request agent the API and return the raw response back.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the `request` fails to complete.
    pub async fn send(&mut self, request: Request<Body>) -> Result<Response<Body>> {
        self.inner
            .ready()
            .await
            .map_err(Error::Hyper)?
            .call(request)
            .await
            .map_err(Error::Hyper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_v2_distribution_returns_200() {
        // Arrange
        let mut client = Client::new(hyper::Client::new());

        let request = Request::builder()
            .uri("http://localhost:5000/v2/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = client.send(request).await.unwrap();

        // Assert
        assert_eq!(hyper::http::StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn test_v2_distribution_basic_returns_200() {
        // Arrange
        let mut client = Client::new(hyper::Client::new());

        let request = Request::builder()
            .uri("http://localhost:5001/v2/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = client.send(request).await.unwrap();

        // Assert
        assert_eq!(hyper::http::StatusCode::UNAUTHORIZED, response.status());
    }

    #[tokio::test]
    async fn test_v2_distribution_token_returns_200() {
        // Arrange
        let mut client = Client::new(hyper::Client::new());

        let request = Request::builder()
            .uri("http://localhost:5002/v2/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = client.send(request).await.unwrap();

        // Assert
        assert_eq!(hyper::http::StatusCode::UNAUTHORIZED, response.status());
    }
}
