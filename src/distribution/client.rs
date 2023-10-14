use hyper::{Body, Request, Response};
use tower::{
    buffer::{Buffer, BufferLayer},
    util::BoxService,
    BoxError, Layer, Service, ServiceExt,
};

/// Client for connecting with a container distribution API.
#[derive(Clone)]
pub struct Client {
    service: Buffer<BoxService<Request<Body>, Response<Body>, BoxError>, Request<Body>>,
}

impl Client {
    /// Create a [`Client`] using a custom `Service` stack.
    pub fn new<S>(service: S) -> Self
    where
        S: Service<Request<Body>, Response = Response<Body>> + Send + 'static,
        S::Future: Send,
        S::Error: Into<BoxError>,
    {
        let service = service.map_err(Into::into);
        let service = BoxService::new(service);
        let service = BufferLayer::new(1).layer(service);
        Self { service }
    }

    /// Perform a raw HTTP request against the API and return the raw response back.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `request` cannot be performed by the underlying service.
    pub async fn send(&self, request: Request<Body>) -> Result<Response<Body>, BoxError> {
        let mut service = self.service.clone();
        service.ready().await?.call(request).await
    }
}

#[cfg(test)]
mod tests {
    use hyper::{Body, Method, Request, Response, StatusCode};
    use tower_test::mock;

    use super::Client;

    #[derive(Debug)]
    struct Error;

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }

    impl std::error::Error for Error {}

    #[tokio::test]
    async fn send_returns_err() {
        // Arrange
        let (service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

        let spawned = tokio::spawn(async move {
            // Arrange
            let (request, send) = handle.next_request().await.expect("service not called");

            send.send_error(Error);

            // Assert
            assert_eq!(request.method(), Method::GET);
            assert_eq!(request.uri(), "http://www.example.com/");
        });

        let client = Client::new(service);

        let request = Request::builder()
            .uri("http://www.example.com/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = client.send(request).await.unwrap_err();

        // Assert
        assert_eq!(response.to_string(), "Error");
        spawned.await.unwrap();
    }

    #[tokio::test]
    async fn send_returns_ok() {
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
            assert_eq!(request.uri(), "http://www.example.com/");
        });

        let client = Client::new(service);

        let request = Request::builder()
            .uri("http://www.example.com/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = client.send(request).await.unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        spawned.await.unwrap();
    }
}
