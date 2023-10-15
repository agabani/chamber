use chamber::{distribution::client::Client, parser::www_authenticate::WwwAuthenticate};
use hyper::body::HttpBody;
use hyper::{Body, Method, Request, StatusCode};
use url::Url;

const BASE_URL: &str = "http://localhost:5002";

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
        "Bearer realm=\"http://127.0.0.1:5003/auth\",service=\"Docker registry\""
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

    let bearer = www_authenticate
        .challenges
        .iter()
        .find(|f| f.auth_scheme == "Bearer")
        .unwrap();

    println!("{bearer:?}");

    let mut url = bearer
        .auth_params
        .iter()
        .find_map(|auth_param| {
            if auth_param.key == "realm" {
                Some(Url::parse(auth_param.value).unwrap())
            } else {
                None
            }
        })
        .unwrap();

    for auth_param in &bearer.auth_params {
        if auth_param.key != "realm" {
            url.query_pairs_mut()
                .append_pair(auth_param.key, auth_param.value);
        }
    }

    let request = Request::builder()
        .uri(url.to_string())
        .header("Authorization", "Basic YWRtaW46cGFzc3dvcmQ=")
        .body(Body::empty())
        .unwrap();

    let response = client.send(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    println!("{response:?}");

    let mut body = response.into_body();

    while let Some(result) = body.data().await {
        let bytes = result.unwrap();
        println!("{bytes:?}");
    }
}
