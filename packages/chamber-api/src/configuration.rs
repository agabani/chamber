use config::{Config, ConfigError, Environment};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub http_server: HttpServer,
}

impl Configuration {
    pub fn load(overrides: &[(&str, &str)]) -> Result<Configuration, ConfigError> {
        let mut config_builder = Config::builder()
            .set_default("http_server.host", "127.0.0.1")?
            .set_default("http_server.port", "8080")?
            .add_source(Environment::with_prefix("APP_").separator("__"));

        for &(key, value) in overrides {
            config_builder = config_builder.set_override(key, value)?;
        }

        config_builder.build()?.try_deserialize()
    }
}

#[derive(serde::Deserialize)]
pub struct HttpServer {
    pub host: String,
    pub port: u16,
}

impl HttpServer {
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.host, self.port))
    }
}
