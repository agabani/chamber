use actix_files::Files;
use actix_web::web;

use crate::configuration::Configuration;

pub fn config(cfg: &mut web::ServiceConfig, configuration: &Configuration) {
    cfg.service(
        Files::new("/", &configuration.static_files.directory)
            .index_file("index.html")
            .use_etag(true)
            .use_last_modified(true),
    );
}
