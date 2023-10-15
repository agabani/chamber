use chamber::{
    distribution::client::Client,
    parser::www_authenticate::{self, WwwAuthenticate},
};
use hyper::{Body, Method, Request, StatusCode};

const BASE_URL: &str = "http://localhost:5001";

#[tokio::test]
async fn workflow() {
    // Arrange
    let client = Client::new(hyper::Client::new());

    // Act
    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("{BASE_URL}/v2/"))
        .body(Body::empty())
        .unwrap();

    let response = client.send(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response
            .headers()
            .get("Www-Authenticate")
            .unwrap()
            .to_str()
            .unwrap(),
        "Basic realm=\"Registry Realm\""
    );

    // Arrange
    let www_authenticate = WwwAuthenticate::parse(
        response
            .headers()
            .get("Www-Authenticate")
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .unwrap();

    println!("{www_authenticate:?}");

    // Act
    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("{BASE_URL}/v2/"))
        .header("Authorization", "Basic YWRtaW46cGFzc3dvcmQ=")
        .body(Body::empty())
        .unwrap();

    let response = client.send(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}
