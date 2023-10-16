use std::borrow::Cow;

use chamber::distribution::{
    api::{Support, SupportRequest},
    authentication::{Credential, Solvers},
    client::Client,
    utils::support,
};
use hyper::StatusCode;

#[tokio::test]
async fn normal() {
    run("http://localhost:5000".into()).await;
}

#[tokio::test]
async fn basic() {
    run("http://localhost:5001".into()).await;
}

#[tokio::test]
async fn bearer() {
    run("http://localhost:5002".into()).await;
}

async fn run(base_url: String) {
    // Arrange
    let client = Client::new(hyper::Client::new());
    let api = Support::new(client.clone());
    let solvers = Solvers::all(client);
    let credential = Credential::UsernamePassword("admin".to_string(), "password".to_string());
    let request = SupportRequest { base_url };

    // Act
    let (response, authentication) = support(
        &api,
        &solvers,
        Some(&credential),
        Cow::Owned(None),
        &request,
    )
    .await
    .unwrap();

    // Assert
    assert_eq!(response.raw.status(), StatusCode::OK);
    println!("{authentication:?}");

    // Act
    let (response, authentication) =
        support(&api, &solvers, Some(&credential), authentication, &request)
            .await
            .unwrap();

    // Assert
    assert_eq!(response.raw.status(), StatusCode::OK);
    println!("{authentication:?}");
}
