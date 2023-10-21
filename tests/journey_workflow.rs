use std::sync::Arc;

use chamber::{
    distribution::{
        authentication::{BasicSolver, BearerSolver, Credential, Solver, UsernamePassword},
        service::Service,
    },
    storage::cache::Cache,
    workflow::CopyWorkflow,
};
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
    let authentication = None;
    let client = hyper::Client::new();
    let credential = Credential::UsernamePassword(UsernamePassword {
        username: "admin".to_string(),
        password: "password".to_string(),
    });
    let solvers: Vec<Arc<dyn Solver>> = vec![
        Arc::new(BasicSolver),
        Arc::new(BearerSolver::new(client.clone())),
    ];

    let workflow = CopyWorkflow {
        blobs: Service::v2_blobs_get(client.clone(), solvers.clone()),
        manifests: Service::v2_manifests_get(client.clone(), solvers.clone()),
        cache: Cache::new("./tmp"),
    };

    workflow
        .execute(
            Url::parse(base_url).unwrap(),
            authentication,
            Some(credential),
            "rust".to_string(),
            "v2".to_string(),
        )
        .await;
}
