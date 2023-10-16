use std::borrow::Cow;

use chamber::distribution::{
    api::{Catalog, CatalogRequest, CatalogResponseBody_},
    authentication::{Credential, Solvers},
    client::Client,
    utils::catalog,
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
    let api = Catalog::new(client.clone());
    let solvers = Solvers::all(client);
    let credential = Credential::UsernamePassword("admin".to_string(), "password".to_string());
    let request = CatalogRequest { base_url };

    // Act
    let (response, authentication) = catalog(
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

    let json = response.body().await.unwrap();

    match json {
        CatalogResponseBody_::Ok(body) => println!("{body:?}"),
        CatalogResponseBody_::Err(body) => println!("{body:?}"),
    }

    // Act
    let (response, authentication) =
        catalog(&api, &solvers, Some(&credential), authentication, &request)
            .await
            .unwrap();

    // Assert
    assert_eq!(response.raw.status(), StatusCode::OK);
    println!("{authentication:?}");

    let json = response.body().await.unwrap();

    match json {
        CatalogResponseBody_::Ok(body) => println!("{body:?}"),
        CatalogResponseBody_::Err(body) => println!("{body:?}"),
    }
}
