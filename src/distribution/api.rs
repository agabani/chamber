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
        base_url: &str,
        authentication: &Option<Authentication>,
    ) -> Result<Response<Body>> {
        let mut request = Request::builder()
            .method(Method::GET)
            .uri(format!("{base_url}/v2/"));

        if let Some(authentication) = authentication {
            let authorization = match authentication {
                Authentication::Basic(authorization) => format!("Basic {authorization}"),
                Authentication::Bearer(bearer) => format!("Bearer {}", bearer.access_token),
            };

            request = request.header("Authorization", authorization);
        }

        let request = request.body(Body::empty()).unwrap();

        let response = self.client.send(request).await.unwrap();

        Ok(response)
    }
}
