use chamber::distribution::{
    api::{SupportAPI, SupportRequest},
    client::Client,
    service,
};
use hyper::{Body, Request};
use tower::{Layer, Service, ServiceExt};

const BASE_URL: &str = "http://localhost:5000";

#[tokio::test]
async fn v2_returns_200() {
    // Arrange
    let mut client = Client::new(hyper::Client::new());

    let request = Request::builder()
        .uri(format!("{BASE_URL}/v2/"))
        .body(Body::empty())
        .unwrap();

    // Act
    let response = client.send(request).await.unwrap();

    // Assert
    assert_eq!(hyper::http::StatusCode::OK, response.status());
}

#[tokio::test]
async fn v2_returns_unit() {
    // Arrange
    let client = Client::new(hyper::Client::new());
    let api = SupportAPI::new(client);
    let request = SupportRequest::new().base_url(BASE_URL);

    // Act
    let response = api.check(request).await.unwrap();

    // Assert
    assert_eq!(hyper::http::StatusCode::OK, response.status());
}

#[tokio::test]
async fn v2_returns_unit_s() {
    // Arrange
    let mut service = service::SupportLayer.layer(hyper::Client::new());

    let request = service::SupportRequest::new().base_uri(BASE_URL);

    // Act
    let response = service.ready().await.unwrap().call(request).await.unwrap();

    // Assert
    assert_eq!(hyper::http::StatusCode::OK, response.status());
}
