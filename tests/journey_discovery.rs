use std::sync::Arc;

use chamber::{
    distribution::{
        api::v2::{SupportRequest, SupportResponse},
        authentication::{BasicSolver, BearerSolver, Solver},
        service::Service,
    },
    service::Service as _,
};

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
    // Arrange
    let client = hyper::Client::new();
    let solvers: Vec<Arc<dyn Solver>> = vec![
        Arc::new(BasicSolver),
        Arc::new(BearerSolver::new(client.clone())),
    ];

    let service = Service::<_, SupportRequest, SupportResponse>::new(client.clone(), solvers);

    // Act
    let request = SupportRequest;

    let response = service.call(request).await.expect("failed to send request");
}
