use chamber::distribution::service::{SupportLayer, SupportRequest};
use hyper::{http::StatusCode, Client};
use tower::{Layer, Service, ServiceExt};

const BASE_URL: &str = "http://localhost:5001";

#[tokio::test]
async fn v2_returns_401() {
    // Arrange
    let mut service = SupportLayer.layer(Client::new());

    let request = SupportRequest::new().base_uri(BASE_URL);

    // Act
    let response = service.ready().await.unwrap().call(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
