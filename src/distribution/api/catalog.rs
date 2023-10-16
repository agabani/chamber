use hyper::{Body, Method, Request, Response, StatusCode};
use url::Url;

use crate::{
    distribution::{
        api::ApiError,
        authentication::Authentication,
        client::Client,
        spec::{CatalogResponseBody, ErrorResponseBody},
        utils::deserialize_response_body,
    },
    Result,
};

///
pub struct Catalog {
    client: Client,
}

impl Catalog {
    ///
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    ///
    pub async fn send(
        &self,
        request: &CatalogRequest,
        authentication: Option<&Authentication>,
    ) -> Result<CatalogResponse, ApiError> {
        let mut url = Url::parse(&request.base_url).map_err(ApiError::Parse)?;
        url.set_path("/v2/_catalog");

        let mut request = Request::builder().method(Method::GET).uri(url.to_string());
        if let Some(authentication) = authentication {
            request = request.header("Authorization", authentication.to_authorization_header());
        }

        let request = request.body(Body::empty()).map_err(ApiError::Http)?;

        let response = self.client.send(request).await.unwrap();

        Ok(CatalogResponse { raw: response })
    }
}

///
pub struct CatalogRequest {
    ///
    pub base_url: String,
}

///
pub struct CatalogResponse {
    ///
    pub raw: Response<Body>,
}

impl CatalogResponse {
    ///
    pub fn new(raw: Response<Body>) -> Self {
        Self { raw }
    }

    ///
    pub async fn body(self) -> Result<CatalogResponseBody_, serde_json::Error> {
        match self.raw.status() {
            StatusCode::OK => deserialize_response_body::<CatalogResponseBody>(self.raw)
                .await
                .map(CatalogResponseBody_::Ok),
            StatusCode::UNAUTHORIZED => deserialize_response_body::<ErrorResponseBody>(self.raw)
                .await
                .map(CatalogResponseBody_::Err),
            status_code => todo!("{status_code}"),
        }
    }
}

///
pub enum CatalogResponseBody_ {
    ///
    Ok(CatalogResponseBody),
    ///
    Err(ErrorResponseBody),
}
