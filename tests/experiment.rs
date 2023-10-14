use std::pin::Pin;

use hyper::{Body, Client, Error, Request, Response, StatusCode};
use tower::retry::{Policy, RetryLayer};
use tower::{Layer, Service, ServiceExt};

#[tokio::test]
async fn authenticated() {
    // Arrange
    let client = Client::new();
    let mut retry = RetryLayer::new(AuthenticationPolicy::new()).layer(client);
    let req = Request::builder()
        .uri("http://localhost:5001/v2/")
        .body(Body::empty())
        .unwrap();

    // Act

    let response = retry.ready().await.unwrap().call(req).await.unwrap();

    // Assert
    // println!("{response:?}");
}

#[tokio::test]
async fn unauthenticated() {
    // Arrange
    let client = Client::new();
    let mut retry = RetryLayer::new(AuthenticationPolicy::new()).layer(client);

    let req = Request::builder()
        .uri("http://localhost:5000/v2/")
        .body(Body::empty())
        .unwrap();

    // Act

    let response = retry.ready().await.unwrap().call(req).await.unwrap();

    // Assert
    // println!("{response:?}");
}

#[derive(Clone)]
struct AuthenticationPolicy {
    attempt: u32,
    credentials: Option<String>,
}

impl AuthenticationPolicy {
    pub fn new() -> Self {
        AuthenticationPolicy {
            attempt: 0,
            credentials: None,
        }
    }
}

impl Policy<Request<Body>, Response<Body>, Error> for AuthenticationPolicy {
    type Future = Pin<Box<dyn std::future::Future<Output = AuthenticationPolicy>>>;

    fn retry(
        &self,
        req: &Request<Body>,
        result: Result<&Response<Body>, &Error>,
    ) -> Option<Self::Future> {
        let attempt = self.attempt;
        println!("retry: {attempt}: {req:?} {result:?}");

        if let Ok(response) = result {
            if response.status() == StatusCode::OK {
                return None;
            }
        }

        if self.attempt >= 3 {
            return None;
        }

        let mut cloned = self.clone();

        let fut = async move {
            cloned.credentials = Some("Basic YWRtaW46cGFzc3dvcmQ=".to_string());
            cloned.attempt += 1;
            cloned
        };

        let f = Box::pin(fut);

        Some(f)
    }

    fn clone_request(&self, req: &Request<Body>) -> Option<Request<Body>> {
        let attempt = self.attempt;
        println!("");
        println!("clone_request: {attempt}: {req:?}");

        let mut request = Request::builder().method(req.method()).uri(req.uri());

        if let Some(credentials) = &self.credentials {
            request = request.header("Authorization", credentials);
        }

        request.body(Body::empty()).ok()
    }
}
