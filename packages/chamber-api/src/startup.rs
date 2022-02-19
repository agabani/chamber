use std::sync::Arc;

use crate::configuration::Configuration;
use crate::routes::{health, static_files};
use crate::tracing::TraceErrorExt;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

/// Configures the HTTP server and dependencies.
///
/// # Panics
///
/// Will panic if configuration cannot be fully loaded due to missing environment variables.
///
/// Will panic if http server cannot bind socket address.
pub fn run(overrides: &[(&str, &str)]) -> (Server, u16, Configuration) {
    let configuration = Configuration::load(overrides)
        .trace_err()
        .expect("Failed to load configuration");

    // configure http listener
    let listener = configuration
        .http_server
        .tcp_listener()
        .trace_err()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    let arc_configuration = Arc::new(configuration.clone());

    // configure server
    let server = HttpServer::new(move || {
        App::new()
            .service(web::scope("/health").configure(health::config))
            .service(web::scope("").configure(|cfg| static_files::config(cfg, &arc_configuration)))
    })
    .listen(listener)
    .trace_err()
    .expect("Failed to bind address")
    .run();

    (server, port, configuration)
}
