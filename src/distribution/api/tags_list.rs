use hyper::{Body, Method, Request, Response, StatusCode};
use url::Url;

use crate::{
    distribution::{api::ApiError, authentication::Authentication, client::Client, spec, utils},
    Result,
};

///
pub struct TagsList {
    client: Client,
}

impl TagsList {
    ///
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    ///
    pub async fn send(
        &self,
        request: &TagsListRequest,
        authentication: Option<&Authentication>,
    ) -> Result<TagsListResponse, ApiError> {
        let mut url = Url::parse(&request.base_url).map_err(ApiError::Parse)?;
        url.set_path(&format!("/v2/{}/tags/list", request.repository));

        let mut request = Request::builder().method(Method::GET).uri(url.to_string());
        if let Some(authentication) = authentication {
            request = request.header("Authorization", authentication.to_authorization_header());
        }

        let request = request.body(Body::empty()).map_err(ApiError::Http)?;

        let response = self.client.send(request).await.unwrap();

        Ok(TagsListResponse { raw: response })
    }
}

///
pub struct TagsListRequest {
    ///
    pub base_url: String,

    ///
    pub repository: String,
}

///
pub struct TagsListResponse {
    ///
    pub raw: Response<Body>,
}

impl TagsListResponse {
    ///
    pub async fn body(self) -> Result<TagsListResponseBody, serde_json::Error> {
        match self.raw.status() {
            StatusCode::OK => utils::deserialize_response_body(self.raw)
                .await
                .map(TagsListResponseBody::Ok),
            status_code => todo!("{status_code}"),
        }
    }
}

///
pub enum TagsListResponseBody {
    ///
    Ok(spec::TagsListResponseBody),

    ///
    Err(spec::ErrorResponseBody),
}
