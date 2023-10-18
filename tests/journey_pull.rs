use std::sync::Arc;

use chamber::{
    distribution::{
        api::v2::manifests_get,
        authentication::{BasicSolver, BearerSolver, Credential, Solver, UsernamePassword},
        service::Service,
    },
    Service as _,
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
    let authentication = None;
    let credential = Credential::UsernamePassword(UsernamePassword {
        username: "admin".to_string(),
        password: "password".to_string(),
    });
    let client = hyper::Client::new();
    let solvers: Vec<Arc<dyn Solver>> = vec![
        Arc::new(BasicSolver),
        Arc::new(BearerSolver::new(client.clone())),
    ];

    // Arrange - Manifest Get
    let service = Service::v2_manifests_get(client.clone(), solvers.clone());

    let request = manifests_get::Request::new(
        Url::parse(base_url).unwrap(),
        authentication,
        Some(credential.clone()),
        "ubuntu".to_string(),
        "v2".to_string(),
        vec![manifests_get::APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V2_JSON.to_string()],
    );

    // Act - Manifest Get
    let response = service.call(request).await.expect("failed to send request");

    // Assert - Manifest Get
    assert_eq!(response.raw().status(), StatusCode::OK);
    let authentication = response.authentication().cloned();
    // let response = response.to_spec().await.unwrap();
    println!("{:?}", authentication);
    // println!("{:?} {:?}", authentication, response);
    println!("");
}
