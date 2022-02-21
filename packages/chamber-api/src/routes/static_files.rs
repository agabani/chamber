use std::path::PathBuf;

use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web,
};

use crate::configuration::Configuration;

pub fn config(cfg: &mut web::ServiceConfig, configuration: &Configuration) {
    cfg.service(
        Files::new("/", &configuration.static_files.directory)
            .index_file("index.html")
            .prefer_utf8(true)
            .use_etag(true)
            .use_last_modified(true)
            .default_handler(|request: ServiceRequest| async move {
                let configuration = request.app_data::<Configuration>().unwrap();
                let index = PathBuf::from(&configuration.static_files.directory).join("index.html");
                let (request, _) = request.into_parts();
                let file = NamedFile::open_async(index).await?;
                let response = file.into_response(&request);
                Ok(ServiceResponse::new(request, response))
            }),
    );
}
