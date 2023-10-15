use chamber::{
    distribution::{
        authentication::{Authentication, Credential, Solvers},
        client::Client,
    },
    parser::www_authenticate::WwwAuthenticate,
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

    // Arrange
    let credential = Credential::UsernamePassword("admin".to_string(), "password".to_string());

    let solvers = Solvers::all(client.clone());

    let www_authenticate = WwwAuthenticate::parse(
        response
            .headers()
            .get("Www-Authenticate")
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .unwrap();

    // Act
    let mut result = None;

    for challenge in &www_authenticate.challenges {
        for solver in solvers.iter() {
            let authentication = solver.solve(challenge, &credential).await.unwrap();

            if let Some(authentication) = authentication {
                let authorization = match authentication {
                    Authentication::Basic(authorization) => format!("Basic {authorization}"),
                    Authentication::Bearer(bearer) => format!("Bearer {}", bearer.access_token),
                };

                let request = Request::builder()
                    .method(Method::GET)
                    .uri(format!("{BASE_URL}/v2/"))
                    .header("Authorization", authorization)
                    .body(Body::empty())
                    .unwrap();

                let response = client.send(request).await.unwrap();

                if response.status() == StatusCode::OK {
                    result = Some(response);
                    break;
                }
            }
        }
    }

    // Assert
    assert_eq!(result.unwrap().status(), StatusCode::OK);
}
