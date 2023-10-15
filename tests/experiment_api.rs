use std::borrow::Cow;

use chamber::distribution::{
    api::Support,
    authentication::{Credential, Solvers},
    client::Client,
    utils::support,
};
use hyper::StatusCode;

#[tokio::test]
async fn normal() {
    run("http://localhost:5000").await;
}

#[tokio::test]
async fn basic() {
    run("http://localhost:5001").await;
}

#[tokio::test]
async fn bearer() {
    run("http://localhost:5002").await;
}

async fn run(base_url: &str) {
    // Arrange
    let client = Client::new(hyper::Client::new());
    let api = Support::new(client.clone());
    let solvers = Solvers::all(client);
    let credential = Credential::UsernamePassword("admin".to_string(), "password".to_string());

    // Act
    let (response, authentication) = support(
        &api,
        &solvers,
        Some(&credential),
        Cow::Owned(None),
        base_url,
    )
    .await
    .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    println!("{authentication:?}");

    // Act
    let (response, authentication) =
        support(&api, &solvers, Some(&credential), authentication, base_url)
            .await
            .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    println!("{authentication:?}");
}
