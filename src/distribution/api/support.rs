use hyper::{Body, Method, Request, Response};
use url::Url;

use crate::{
    distribution::{api::ApiError, authentication::Authentication, client::Client},
    Result,
};

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
    ) -> Result<SupportResponse, ApiError> {
        let mut url = Url::parse(&request.base_url).map_err(ApiError::Parse)?;
        url.set_path("/v2/");

        let mut request = Request::builder().method(Method::GET).uri(url.to_string());
        if let Some(authentication) = authentication {
            request = request.header("Authorization", authentication.to_authorization_header());
        }

        let request = request.body(Body::empty()).map_err(ApiError::Http)?;

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
