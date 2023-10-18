use std::sync::Arc;

use chamber::{
    distribution::{
        api::v2::{CatalogRequest, CatalogResponse, SupportRequest, SupportResponse},
        authentication::{BasicSolver, BearerSolver, Credential, Solver, UsernamePassword},
        service::Service,
    },
    service::Service as _,
};
use hyper::StatusCode;
use url::Url;

#[tokio::test]
async fn distribution() {
    run("http://localhost:5000").await;
}

#[tokio::test]
async fn distribution_basic() {
    run("http://localhost:5001").await;
}

#[tokio::test]
async fn distribution_bearer() {
    run("http://localhost:5002").await;
}

async fn run(base_url: &str) {
    // Setup
    let credential = Credential::UsernamePassword(UsernamePassword {
        username: "admin".to_string(),
        password: "password".to_string(),
    });
    let client = hyper::Client::new();
    let solvers: Vec<Arc<dyn Solver>> = vec![
        Arc::new(BasicSolver),
        Arc::new(BearerSolver::new(client.clone())),
    ];

    // Arrange
    let service =
        Service::<_, SupportRequest, SupportResponse>::new(client.clone(), solvers.clone());

    let request = SupportRequest::new(
        Url::parse(base_url).unwrap(),
        None,
        Some(credential.clone()),
    );

    // Act
    let response = service.call(request).await.expect("failed to send request");

    // Assert
    println!("{:?} {:?}", response.authentication(), response.raw());

    assert_eq!(response.raw().status(), StatusCode::OK);

    // Arrange
    let service =
        Service::<_, CatalogRequest, CatalogResponse>::new(client.clone(), solvers.clone());

    let request = CatalogRequest::new(
        Url::parse(base_url).unwrap(),
        None,
        Some(credential.clone()),
    );

    // Act
    let response = service.call(request).await.expect("failed to send request");

    // Assert
    assert_eq!(response.raw().status(), StatusCode::OK);
    let response = response.to_json().await.unwrap();
    println!("{:?}", response);
}
