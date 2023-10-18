use std::sync::Arc;

use chamber::{
    distribution::{
        api::v2::{catalog, support, tags_list},
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

    // Arrange - Support
    let service =
        Service::<_, support::Request, support::Response>::new(client.clone(), solvers.clone());

    let request = support::Request::new(
        Url::parse(base_url).unwrap(),
        authentication,
        Some(credential.clone()),
    );

    // Act - Support
    let response = service.call(request).await.expect("failed to send request");

    // Assert - Support
    assert_eq!(response.raw().status(), StatusCode::OK);
    let authentication = response.authentication().cloned();
    let response = response.to_spec().await.unwrap();
    println!("{:?} {:?}", authentication, response);
    println!("");

    // Arrange - Catalog
    let service =
        Service::<_, catalog::Request, catalog::Response>::new(client.clone(), solvers.clone());

    let request = catalog::Request::new(
        Url::parse(base_url).unwrap(),
        authentication,
        Some(credential.clone()),
    );

    // Act - Catalog
    let response = service.call(request).await.expect("failed to send request");

    // Assert - Catalog
    assert_eq!(response.raw().status(), StatusCode::OK);
    let authentication = response.authentication().cloned();
    let response = response.to_spec().await.unwrap();
    println!("{:?} {:?}", authentication, response);
    println!("");

    // Arrange - TagsList
    let service =
        Service::<_, tags_list::Request, tags_list::Response>::new(client.clone(), solvers.clone());

    let request = tags_list::Request::new(
        Url::parse(base_url).unwrap(),
        response.repositories.first().unwrap().clone(),
        authentication,
        Some(credential.clone()),
    );

    // Act - TagsList
    let response = service.call(request).await.expect("failed to send request");

    // Assert - TagsList
    assert_eq!(response.raw().status(), StatusCode::OK);
    let authentication = response.authentication().cloned();
    let response = response.to_spec().await.unwrap();
    println!("{:?} {:?}", authentication, response);
    println!("");

    // Arrange - TagsList Not Found
    let service =
        Service::<_, tags_list::Request, tags_list::Response>::new(client.clone(), solvers.clone());

    let request = tags_list::Request::new(
        Url::parse(base_url).unwrap(),
        "not_found".to_string(),
        authentication,
        Some(credential.clone()),
    );

    // Act - TagsList Not Found
    let response = service.call(request).await.expect("failed to send request");

    // Assert - TagsList Not Found
    assert_eq!(response.raw().status(), StatusCode::NOT_FOUND);
    let authentication = response.authentication().cloned();
    let response = response.to_spec().await.unwrap();
    println!("{:?} {:?}", authentication, response);
    println!("");
}
