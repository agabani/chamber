use std::borrow::Cow;

use chamber::distribution::{api, authentication, client, utils};

#[tokio::test]
async fn distribution() {
    run("http://localhost:5000".into()).await;
}

#[tokio::test]
async fn distribution_basic() {
    run("http://localhost:5001".into()).await;
}

#[tokio::test]
async fn distribution_bearer() {
    run("http://localhost:5002".into()).await;
}

async fn run(base_url: String) {
    // Setup
    let client = client::Client::new(hyper::Client::new());

    let credential =
        authentication::Credential::UsernamePassword("admin".to_string(), "password".to_string());
    let solvers = authentication::Solvers::all(client.clone());

    // # Support
    let authentication = {
        // Arrange
        let api = api::Support::new(client.clone());

        let request = api::SupportRequest {
            base_url: base_url.clone(),
        };

        // Act
        let (response, authentication) = utils::support(
            &api,
            &solvers,
            Some(&credential),
            Cow::Owned(None),
            &request,
        )
        .await
        .unwrap();

        // Assert
        assert_eq!(response.raw.status(), hyper::StatusCode::OK);

        authentication
    };

    // # Catalog
    let authentication = {
        // Arrange
        let api = api::Catalog::new(client.clone());

        let request = api::CatalogRequest {
            base_url: base_url.clone(),
        };

        // Act
        let (response, authentication) =
            utils::catalog(&api, &solvers, Some(&credential), authentication, &request)
                .await
                .unwrap();

        // Assert
        assert_eq!(response.raw.status(), hyper::StatusCode::OK);

        // Act
        let response = response.body().await.unwrap();

        // Assert
        match response {
            api::CatalogResponseBody_::Ok(response) => {
                assert_eq!(response.repositories.len(), 1);
                assert_eq!(response.repositories[0], "ubuntu");
            }
            api::CatalogResponseBody_::Err(error) => panic!("{error:?}"),
        }

        authentication
    };
}
