use hyper::{Body, Method, Request, Response, StatusCode};

use crate::{
    distribution::{api::CatalogResponseBody, utils::deserialize_response_body},
    Result,
};

use super::{
    super::{authentication::Authentication, client::Client},
    ErrorResponseBody,
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
    ) -> Result<CatalogResponse> {
        let mut request = Request::builder()
            .method(Method::GET)
            .uri(format!("{}/v2/_catalog", request.base_url));

        if let Some(authentication) = authentication {
            let authorization = match authentication {
                Authentication::Basic(authorization) => format!("Basic {authorization}"),
                Authentication::Bearer(bearer) => format!("Bearer {}", bearer.access_token),
            };

            request = request.header("Authorization", authorization);
        }

        let request = request.body(Body::empty()).unwrap();

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
