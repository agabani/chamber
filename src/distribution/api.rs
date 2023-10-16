use hyper::{Body, Method, Request, Response};

use crate::Result;

use super::{authentication::Authentication, client::Client};

///
pub struct Support {
    client: Client,
}

impl Support {
    ///
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    ///
    pub async fn send(
        &self,
        request: &SupportRequest,
        authentication: Option<&Authentication>,
    ) -> Result<SupportResponse> {
        let mut request = Request::builder()
            .method(Method::GET)
            .uri(format!("{}/v2/", request.base_url));

        if let Some(authentication) = authentication {
            let authorization = match authentication {
                Authentication::Basic(authorization) => format!("Basic {authorization}"),
                Authentication::Bearer(bearer) => format!("Bearer {}", bearer.access_token),
            };

            request = request.header("Authorization", authorization);
        }

        let request = request.body(Body::empty()).unwrap();

        let response = self.client.send(request).await.unwrap();

        Ok(SupportResponse { raw: response })
    }
}

///
pub struct SupportRequest {
    ///
    pub base_url: String,
}

///
pub struct SupportResponse {
    ///
    pub raw: Response<Body>,
}
