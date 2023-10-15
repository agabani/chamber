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
    // Arrange
    let client = Client::new(hyper::Client::new());

    let support = Support::new(client);

    // Act
    let response = support.send("http://localhost:5000", &None).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn __() {
    // Arrange
    let client = Client::new(hyper::Client::new());

    let api = Support::new(client.clone());

    let solvers = Solvers::all(client);

    let credential = Credential::UsernamePassword("admin".to_string(), "password".to_string());

    // Act
    let (response, authentication) = support(
        api,
        &solvers,
        &Some(credential),
        Cow::Owned(None),
        "http://localhost:5001",
    )
    .await
    .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    println!("{authentication:?}");
}
