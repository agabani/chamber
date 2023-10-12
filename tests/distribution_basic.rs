use chamber::distribution::client::Client;
use hyper::{Body, Request};

const BASE_URL: &str = "http://localhost:5001";

#[tokio::test]
async fn v2_returns_401() {
    // Arrange
    let mut client = Client::new(hyper::Client::new());

    let request = Request::builder()
        .uri(format!("{BASE_URL}/v2/"))
        .body(Body::empty())
        .unwrap();

    // Act
    let response = client.send(request).await.unwrap();

    // Assert
    assert_eq!(hyper::http::StatusCode::UNAUTHORIZED, response.status());
}
