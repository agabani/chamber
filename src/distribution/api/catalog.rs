use hyper::{Body, Method, Request, Response};

use crate::Result;

use super::super::{authentication::Authentication, client::Client};

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
