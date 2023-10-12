use hyper::{Body, Request, Response};
use tower::{Service, ServiceExt};

use crate::{BoxError, Error, Result};

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
    S: Service<Request<Body>, Response = Response<Body>>,
    S::Error: Into<BoxError>,
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
            .map_err(|err| Error::from(err.into()))?
            .call(request)
            .await
            .map_err(|err| Error::from(err.into()))
    }
}

#[cfg(test)]
mod tests {
    use tower_test::mock;

    use super::*;

    #[tokio::test]
    async fn v2_returns_200() {
        // Arrange
        let (service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

        let spawned = tokio::spawn(async move {
            let (request, send) = handle.next_request().await.expect("service not called");
            assert_eq!(request.method(), hyper::http::Method::GET);
            assert_eq!(request.uri(), "http://localhost:5000/v2/");
            let response = Response::builder()
                .status(hyper::http::StatusCode::OK)
                .body(Body::empty())
                .unwrap();
            send.send_response(response);
        });

        let mut client = Client::new(service);

        let request = Request::builder()
            .uri("http://localhost:5000/v2/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = client.send(request).await.unwrap();

        // Assert
        assert_eq!(hyper::http::StatusCode::OK, response.status());
        spawned.await.unwrap();
    }

    #[tokio::test]
    async fn v2_returns_401() {
        // Arrange
        let (service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

        let spawned = tokio::spawn(async move {
            let (request, send) = handle.next_request().await.expect("service not called");
            assert_eq!(request.method(), hyper::http::Method::GET);
            assert_eq!(request.uri(), "http://localhost:5000/v2/");
            let response = Response::builder()
                .status(hyper::http::StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap();
            send.send_response(response);
        });

        let mut client = Client::new(service);

        let request = Request::builder()
            .uri("http://localhost:5000/v2/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = client.send(request).await.unwrap();

        // Assert
        assert_eq!(hyper::http::StatusCode::UNAUTHORIZED, response.status());
        spawned.await.unwrap();
    }
}
