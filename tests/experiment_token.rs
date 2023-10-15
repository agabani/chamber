use chamber::{
    distribution::{
        authentication::{Authentication, BearerSolver, Credential, Solver},
        client::Client,
    },
    parser::www_authenticate::WwwAuthenticate,
};
use hyper::{Body, Method, Request, StatusCode};

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

    let solver = BearerSolver::new(client.clone());

    let authentication = solver
        .solve(
            &www_authenticate.challenges[0],
            &Credential::UsernamePassword("admin".to_string(), "password".to_string()),
        )
        .await
        .unwrap()
        .unwrap();

    let authorization = match authentication {
        Authentication::Basic(_) => todo!(),
        Authentication::Bearer(bearer) => format!("Bearer {}", bearer.access_token),
    };

    // Act
    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("{BASE_URL}/v2/"))
        .header("Authorization", authorization)
        .body(Body::empty())
        .unwrap();

    let response = client.send(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}
